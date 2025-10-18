use crate::{
    models::chart_view::chart::chart_model::ChartModel,
    services::chart_processor::{ChartProcessingService, FftFilterType},
    shared::{
        commands::chart_view::ChartViewCommands,
        constants::{
            chart_view::{DEFAULT_CHART_X_MOVE, DEFAULT_CHART_ZOOM_MULTIPLIER},
            command::DEFAULT_COMMAND_PREFIX,
        },
        errors::commands::CommandError,
    },
    states::{app::ApplicationState, chart_view::ChartViewState},
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
use std::{cell::RefCell, rc::Rc, str::FromStr};

pub struct ChartViewComponent {
    state: ChartViewState,
    service: ChartProcessingService,
    app_state: Rc<RefCell<ApplicationState>>,
}

impl ChartViewComponent {
    pub fn new(app_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self {
            service: ChartProcessingService::new(),
            state: ChartViewState::new(),
            app_state,
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.state.chart_move(true, DEFAULT_CHART_X_MOVE);
            }
            KeyCode::Right => {
                self.state.chart_move(false, DEFAULT_CHART_X_MOVE);
            }
            KeyCode::Up => self.state.chart_scale(true, DEFAULT_CHART_ZOOM_MULTIPLIER),
            KeyCode::Down => self.state.chart_scale(false, DEFAULT_CHART_ZOOM_MULTIPLIER),
            _ => {}
        }
    }

    pub fn update_from_state(&mut self) -> Result<()> {
        let mut state_borrow = self.app_state.borrow_mut();
        if let Some(cmd) = state_borrow.command() {
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
                return Err(CommandError::EmptyCommand.into());
            }
            if let Ok(command) = ChartViewCommands::from_str(args[0]) {
                match command {
                    ChartViewCommands::ZoomIn => {
                        if let Some(multiplier_arg) = args.get(1) {
                            self.state.chart_scale(
                                false,
                                multiplier_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*multiplier_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::ZoomOut => {
                        if let Some(multiplier_arg) = args.get(1) {
                            self.state.chart_scale(
                                true,
                                multiplier_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*multiplier_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::MoveLeft => {
                        if let Some(points_arg) = args.get(1) {
                            self.state.chart_move(
                                true,
                                points_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::MoveRight => {
                        if let Some(points_arg) = args.get(1) {
                            self.state.chart_move(
                                false,
                                points_arg.parse::<f64>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::FastFourierTransform => {
                        let current_plot = self.state.current_chart();
                        let current_plot_borrow = current_plot.borrow();
                        let plot = ChartModel::new(
                            self.service.fft_forward(&current_plot_borrow),
                            current_plot_borrow.metadata.chart_display_type,
                            current_plot_borrow.sample_rate,
                            &current_plot_borrow.metadata.title,
                            Some(&current_plot_borrow.metadata.description), // TODO: Change description
                        );
                        state_borrow.add_chart(plot);
                    }
                    ChartViewCommands::ShortTimeFourierTransform => {
                        if let (Some(window_size), Some(hop_size)) = (args.get(1), args.get(2)) {
                            let current_plot = self.state.current_chart();
                            let current_plot_borrow = current_plot.borrow();
                            let plot = ChartModel::new(
                                self.service.stft_forward(
                                    &current_plot_borrow,
                                    window_size.parse::<usize>().map_err(|_| {
                                        CommandError::InvalidArguments(String::from(*window_size))
                                    })?,
                                    hop_size.parse::<usize>().map_err(|_| {
                                        CommandError::InvalidArguments(String::from(*hop_size))
                                    })?,
                                ),
                                current_plot_borrow.metadata.chart_display_type,
                                current_plot_borrow.sample_rate,
                                &current_plot_borrow.metadata.title,
                                Some(&current_plot_borrow.metadata.description), // TODO: Change description
                            );
                            state_borrow.add_chart(plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::FftFilterLowPass => {
                        if let Some(points_arg) = args.get(1) {
                            let current_plot = self.state.current_chart();
                            let current_plot_borrow = current_plot.borrow();
                            let plot = ChartModel::new(
                                self.service.apply_fft_filter(
                                    &current_plot_borrow,
                                    FftFilterType::LowPass(points_arg.parse::<f64>().map_err(
                                        |_| {
                                            CommandError::InvalidArguments(String::from(
                                                *points_arg,
                                            ))
                                        },
                                    )?),
                                ),
                                current_plot_borrow.metadata.chart_display_type,
                                current_plot_borrow.sample_rate,
                                &current_plot_borrow.metadata.title,
                                Some(&current_plot_borrow.metadata.description), // TODO: Change description
                            );
                            state_borrow.add_chart(plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::FftFilterHighPass => {
                        if let Some(points_arg) = args.get(1) {
                            let current_plot = self.state.current_chart();
                            let current_plot_borrow = current_plot.borrow();
                            let plot = ChartModel::new(
                                self.service.apply_fft_filter(
                                    &current_plot_borrow,
                                    FftFilterType::HighPass(points_arg.parse::<f64>().map_err(
                                        |_| {
                                            CommandError::InvalidArguments(String::from(
                                                *points_arg,
                                            ))
                                        },
                                    )?),
                                ),
                                current_plot_borrow.metadata.chart_display_type,
                                current_plot_borrow.sample_rate,
                                &current_plot_borrow.metadata.title,
                                Some(&current_plot_borrow.metadata.description), // TODO: Change description
                            );
                            state_borrow.add_chart(plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::FftFilterBandPass => {
                        if let (Some(low_band), Some(high_band)) = (args.get(1), args.get(2)) {
                            let current_plot = self.state.current_chart();
                            let current_plot_borrow = current_plot.borrow();
                            let plot = ChartModel::new(
                                self.service.apply_fft_filter(
                                    &current_plot_borrow,
                                    FftFilterType::BandPass(
                                        low_band.parse::<f64>().map_err(|_| {
                                            CommandError::InvalidArguments(String::from(*low_band))
                                        })?,
                                        low_band.parse::<f64>().map_err(|_| {
                                            CommandError::InvalidArguments(String::from(*high_band))
                                        })?,
                                    ),
                                ),
                                current_plot_borrow.metadata.chart_display_type,
                                current_plot_borrow.sample_rate,
                                &current_plot_borrow.metadata.title,
                                Some(&current_plot_borrow.metadata.description), // TODO: Change description
                            );
                            state_borrow.add_chart(plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::FftFilterBandStop => {
                        if let (Some(low_band), Some(high_band)) = (args.get(1), args.get(2)) {
                            let current_plot = self.state.current_chart();
                            let current_plot_borrow = current_plot.borrow();
                            let plot = ChartModel::new(
                                self.service.apply_fft_filter(
                                    &current_plot_borrow,
                                    FftFilterType::BandStop(
                                        low_band.parse::<f64>().map_err(|_| {
                                            CommandError::InvalidArguments(String::from(*low_band))
                                        })?,
                                        low_band.parse::<f64>().map_err(|_| {
                                            CommandError::InvalidArguments(String::from(*high_band))
                                        })?,
                                    ),
                                ),
                                current_plot_borrow.metadata.chart_display_type,
                                current_plot_borrow.sample_rate,
                                &current_plot_borrow.metadata.title,
                                Some(&current_plot_borrow.metadata.description), // TODO: Change description
                            );
                            state_borrow.add_chart(plot);
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartViewCommands::HaarWaveletTransform => {
                        let current_plot = self.state.current_chart();
                        let current_plot_borrow = current_plot.borrow();
                        let plot = ChartModel::new(
                            self.service.haar_wavelet_transform(&current_plot_borrow),
                            current_plot_borrow.metadata.chart_display_type,
                            current_plot_borrow.sample_rate,
                            &current_plot_borrow.metadata.title,
                            Some(&current_plot_borrow.metadata.description), // TODO: Change description
                        );
                        state_borrow.add_chart(plot);
                    }
                };
                state_borrow.set_command(None);
            }
        }
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let current_dataset = &self.app_state.borrow().get_current_chart();
        self.state.set_current_chart(current_dataset.clone());
        let current_dataset_borrow = current_dataset.borrow();
        let pure_coordinates = current_dataset_borrow.data_to_pure_coordinates();
        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::HalfBlock)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(current_dataset_borrow.metadata.chart_display_type)
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
