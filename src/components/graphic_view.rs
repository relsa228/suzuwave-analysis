use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols::{self, Marker},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType,
        canvas::{self, Canvas, Context},
    },
};
use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc, str::FromStr};

use crate::{
    clients::{files::vibric::VibricReadingClient, traits::file_read_only::FileReadOnly},
    models::files::file_types::FileType,
    shared::{
        commands::graphic_view::GraphicViewCommands,
        constants::graphic_view::{DEFAULT_PLOT_X_MOVE, DEFAULT_PLOT_ZOOM_MULTIPLIER},
        errors::{commands::CommandError, files::FileError},
    },
    states::{app::ApplicationState, graphic_view::GraphicViewState},
};

pub struct GraphicViewComponent {
    state: GraphicViewState,
    file_parsers: HashMap<FileType, Box<dyn FileReadOnly>>,
}

impl GraphicViewComponent {
    pub fn new() -> Self {
        let mut file_parsers: HashMap<FileType, Box<dyn FileReadOnly>> = HashMap::new();
        file_parsers.insert(FileType::Vibric, Box::new(VibricReadingClient::new()));

        // Testing
        let mut instant = Self {
            file_parsers: file_parsers,
            state: GraphicViewState::new(),
        };
        let _ =instant.add_plot_from_file(Path::new("/Users/relsa/Downloads/XZ/ОЭД_в_СС/Данные/OBRAZCY_SIGNALOV/01_04_2009_12_07_01_ch1_datafile.bin"));

        instant
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
                self.state.plot_scale(true, DEFAULT_PLOT_ZOOM_MULTIPLIER);
            }
            KeyCode::Down => {
                self.state.plot_scale(false, DEFAULT_PLOT_ZOOM_MULTIPLIER);
            }
            _ => {}
        }
    }

    pub fn update_from_state(&mut self, state: Rc<RefCell<ApplicationState>>) -> Result<()> {
        let cmd = state.borrow().command.clone();
        if let Some(cmd) = cmd {
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() {
                return Err(CommandError::EmptyCommand.into());
            }
            match GraphicViewCommands::from_str(args[0])
                .map_err(|_| CommandError::CommandSyntax(String::from(args[0])))?
            {
                GraphicViewCommands::OpenFile => {
                    if let Some(path_arg) = args.get(1) {
                        let file_path = Path::new(path_arg);
                        if !file_path.exists() {
                            return Err(
                                CommandError::InvalidArguments(String::from(*path_arg)).into()
                            );
                        }
                        self.add_plot_from_file(&file_path)?;
                    } else {
                        return Err(CommandError::NotEnoughArguments.into());
                    }
                }

                GraphicViewCommands::ZoomIn => {
                    if let Some(multiplier_arg) = args.get(1) {
                        self.state.plot_scale(
                            true,
                            multiplier_arg.parse::<f64>().map_err(|_| {
                                CommandError::InvalidArguments(String::from(*multiplier_arg))
                            })?,
                        );
                    } else {
                        return Err(CommandError::NotEnoughArguments.into());
                    }
                }
                GraphicViewCommands::ZoomOut => {
                    if let Some(multiplier_arg) = args.get(1) {
                        self.state.plot_scale(
                            false,
                            multiplier_arg.parse::<f64>().map_err(|_| {
                                CommandError::InvalidArguments(String::from(*multiplier_arg))
                            })?,
                        );
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
            }
            state.borrow_mut().command = None;
        }
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let current_dataset = &self.state.current_dataset().data_to_pure_coordinates();
        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::HalfBlock)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(GraphType::Line)
                .data(&current_dataset),
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

    fn add_plot_from_file(&mut self, path: &Path) -> Result<()> {
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
            context.print(val, self.state.y_min(), format!("{:.2}", val));
        });

        let step = (self.state.y_max() - self.state.y_min()) / (steps) as f64;
        (1..steps).for_each(|i| {
            let val = self.state.y_min() + step * i as f64;
            context.print(self.state.x_min(), val, format!("{:.2}", val));
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
