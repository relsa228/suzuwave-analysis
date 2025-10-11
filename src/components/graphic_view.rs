use crate::{
    clients::{files::vibric::VibricReadingClient, traits::file_read_only::FileReadOnly},
    models::files::file_types::FileType,
    services::graphic_process::{FftFilterType, GraphicProcessService},
    shared::{
        commands::graphic_view::GraphicViewCommands,
        constants::{
            command::DEFAULT_COMMAND_PREFIX,
            graphic_view::{DEFAULT_PLOT_X_MOVE, DEFAULT_PLOT_ZOOM_MULTIPLIER},
        },
        errors::{commands::CommandError, files::FileError},
    },
    states::{app::ApplicationState, graphic_view::GraphicViewState},
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols::{self, Marker},
    widgets::{
        Axis, Block, Borders, Chart, Dataset,
        canvas::{self, Canvas, Context},
    },
};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

pub struct GraphicViewComponent {
    state: GraphicViewState,
    service: GraphicProcessService,
    app_state: Rc<RefCell<ApplicationState>>,
    file_parsers: HashMap<FileType, Box<dyn FileReadOnly>>,
}

impl GraphicViewComponent {
    pub fn new(
        initial_signal_file: Option<PathBuf>,
        app_state: Rc<RefCell<ApplicationState>>,
    ) -> Self {
        let mut file_parsers: HashMap<FileType, Box<dyn FileReadOnly>> = HashMap::new();
        file_parsers.insert(FileType::Vibric, Box::new(VibricReadingClient::new()));
        let mut instance = Self {
            file_parsers: file_parsers,
            service: GraphicProcessService::new(),
            state: GraphicViewState::new(),
            app_state,
        };
        if let Some(file) = initial_signal_file {
            let _ = instance.add_plot_from_file(file);
        }
        instance
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.state.plot_move(true, DEFAULT_PLOT_X_MOVE);
            }
            KeyCode::Right => {
                self.state.plot_move(false, DEFAULT_PLOT_X_MOVE);
            }
            KeyCode::Up => {
                if let Err(err) = self.state.plot_scale(true, DEFAULT_PLOT_ZOOM_MULTIPLIER) {
                    self.app_state.borrow_mut().set_error(Some(err));
                }
            }
            KeyCode::Down => {
                if let Err(err) = self.state.plot_scale(false, DEFAULT_PLOT_ZOOM_MULTIPLIER) {
                    self.app_state.borrow_mut().set_error(Some(err));
                }
            }
            _ => {}
        }
    }

    pub fn update_from_state(&mut self, state: Rc<RefCell<ApplicationState>>) -> Result<()> {
        let mut state_borrow = state.borrow_mut();
        if let Some(cmd) = state_borrow.command() {
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
                return Err(CommandError::EmptyCommand.into());
            }
            if let Ok(command) = GraphicViewCommands::from_str(args[0]) {
                match command {
                    GraphicViewCommands::OpenFile => {
                        if let Some(path_arg) = args.get(1) {
                            let file_path = Path::new(path_arg);
                            if !file_path.exists() {
                                return Err(CommandError::InvalidArguments(String::from(
                                    *path_arg,
                                ))
                                .into());
                            }
                            self.add_plot_from_file(file_path.to_path_buf())?;
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::ZoomIn => {
                        if let Some(multiplier_arg) = args.get(1) {
                            self.state.plot_scale(
                                false,
                                multiplier_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*multiplier_arg))
                                })?,
                            )?;
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::ZoomOut => {
                        if let Some(multiplier_arg) = args.get(1) {
                            self.state.plot_scale(
                                true,
                                multiplier_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*multiplier_arg))
                                })?,
                            )?;
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::MoveLeft => {
                        if let Some(points_arg) = args.get(1) {
                            self.state.plot_move(
                                true,
                                points_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::MoveRight => {
                        if let Some(points_arg) = args.get(1) {
                            self.state.plot_move(
                                false,
                                points_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::CloseWorkingView => {
                        self.state.delete_current_plot();
                    }
                    GraphicViewCommands::SwitchWorkingView => {
                        if let Some(points_arg) = args.get(1) {
                            self.state
                                .change_current_plot(points_arg.parse::<u32>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    GraphicViewCommands::FastFourierTransform => {
                        let mut current_plot = self.state.current_dataset();
                        current_plot.data = self.service.fft_forward(current_plot.data);
                        self.state.add_plot(current_plot);
                    }
                    GraphicViewCommands::FftFilterLowPass => {
                        if let Some(points_arg) = args.get(1) {
                            let mut current_plot = self.state.current_dataset();
                            current_plot.data = self.service.apply_filter(
                                &current_plot,
                                FftFilterType::LowPass(points_arg.parse::<f64>().map_err(
                                    |_| CommandError::InvalidArguments(String::from(*points_arg)),
                                )?),
                            );
                            self.state.add_plot(current_plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                };
                state_borrow.set_command(None);
            }
        }
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let current_dataset = &self.state.current_dataset();
        let pure_coordinates = current_dataset.data_to_pure_coordinates();
        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::HalfBlock)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(current_dataset.graph_type())
                .data(&pure_coordinates),
        ];

        let chart = Chart::new(datasets)
            .block(Block::default().borders(Borders::NONE))
            .x_axis(Axis::default().bounds([self.state.x_min(), self.state.x_max()]))
            .y_axis(Axis::default().bounds([self.state.y_min(), self.state.y_max()]));

        let canvas = Canvas::default()
            .block(Block::new())
            .marker(Marker::Braille)
            .x_bounds([self.state.x_min(), self.state.x_max()])
            .y_bounds([self.state.y_min(), self.state.y_max()])
            .paint(|context| {
                context.draw(self.state.canvas_style().map());
                self.canvas_generate_labels(context, self.state.canvas_style().canvas_steps());
                self.canvas_generate_grid(context, self.state.canvas_style().canvas_steps());
            });
        f.render_widget(canvas, rect);
        f.render_widget(chart, rect);
    }

    fn add_plot_from_file(&mut self, path: PathBuf) -> Result<()> {
        if let Some(extension) = path.extension() {
            let parser = self
                .file_parsers
                .get(&FileType::from_str(
                    &extension.to_str().ok_or(FileError::ExtensionParseError)?,
                )?)
                .ok_or(FileError::UnsupportedType)?;
            let data =
                parser.parse_signal_file(path.to_str().ok_or(FileError::PathParseError)?, 0)?;
            self.state.add_plot(data);
        }
        Ok(())
    }

    fn canvas_generate_labels(&self, context: &mut Context<'_>, steps: u32) {
        let step = (self.state.x_max() - self.state.x_min()) / (steps) as f64;
        (1..steps).for_each(|i| {
            let val = self.state.x_min() + step * i as f64;
            context.print(val, self.state.y_min(), format!("{:.4}", val));
        });

        let step = (self.state.y_max() - self.state.y_min()) / (steps) as f64;
        (1..steps).for_each(|i| {
            let val = self.state.y_min() + step * i as f64;
            context.print(self.state.x_min(), val, format!("{:.4}", val));
        });
    }

    fn canvas_generate_grid(&self, context: &mut Context<'_>, steps: u32) {
        let step = (self.state.x_max() - self.state.x_min()) / (steps) as f64;
        (0..steps).for_each(|i| {
            let val = self.state.x_min() + step * i as f64;
            context.draw(&canvas::Line::new(
                val,
                self.state.y_min(),
                val,
                self.state.y_max(),
                self.state.canvas_style().canvas_color(),
            ));
        });

        let step = (self.state.y_max() - self.state.y_min()) / (steps) as f64;
        (0..steps).for_each(|i| {
            let val = self.state.y_min() + step * i as f64;
            context.draw(&canvas::Line::new(
                self.state.x_min(),
                val,
                self.state.x_max(),
                val,
                self.state.canvas_style().canvas_color(),
            ));
        });
    }
}
