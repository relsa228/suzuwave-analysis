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
        let Some(cmd) = state_borrow.command() else {
            return Ok(());
        };
        let args = cmd.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
            return Err(CommandError::EmptyCommand.into());
        }
        let Ok(command) = ChartViewCommands::from_str(args[0]) else {
            return Ok(());
        };
        match command {
            ChartViewCommands::ZoomIn => {
                let Some(multiplier_arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let multiplier_arg = multiplier_arg
                    .parse::<f64>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*multiplier_arg)))?;
                self.state.chart_scale(false, multiplier_arg);
            }
            ChartViewCommands::ZoomOut => {
                let Some(multiplier_arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let multiplier_arg = multiplier_arg
                    .parse::<f64>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*multiplier_arg)))?;
                self.state.chart_scale(true, multiplier_arg);
            }
            ChartViewCommands::MoveLeft => {
                let Some(points_arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let points_arg = points_arg
                    .parse::<f64>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*points_arg)))?;
                self.state.chart_move(true, points_arg);
            }
            ChartViewCommands::MoveRight => {
                let Some(points_arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let points_arg = points_arg
                    .parse::<f64>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*points_arg)))?;
                self.state.chart_move(false, points_arg);
            }
            ChartViewCommands::FastFourierTransform => {
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.fft_forward(&current_chart_borrow),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::ShortTimeFourierTransform => {
                let (Some(window_size), Some(hop_size)) = (args.get(1), args.get(2)) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let current_chart_borrow = current_chart.borrow();
                let window_size = window_size
                    .parse::<usize>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*window_size)))?;
                let hop_size = hop_size
                    .parse::<usize>()
                    .map_err(|_| CommandError::InvalidArguments(String::from(*hop_size)))?;
                let chart = ChartModel::new(
                    self.service
                        .stft_forward(&current_chart_borrow, window_size, hop_size)?,
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::FftFilterLowPass => {
                let Some(arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let filter = FftFilterType::LowPass(
                    arg.parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*arg)))?,
                );
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.apply_fft_filter(&current_chart_borrow, filter),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::FftFilterHighPass => {
                let Some(arg) = args.get(1) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let filter = FftFilterType::HighPass(
                    arg.parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*arg)))?,
                );
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.apply_fft_filter(&current_chart_borrow, filter),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::FftFilterBandPass => {
                let (Some(low_band), Some(high_band)) = (args.get(1), args.get(2)) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let filter = FftFilterType::BandPass(
                    low_band
                        .parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*low_band)))?,
                    high_band
                        .parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*high_band)))?,
                );
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.apply_fft_filter(&current_chart_borrow, filter),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::FftFilterBandStop => {
                let (Some(low_band), Some(high_band)) = (args.get(1), args.get(2)) else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let filter = FftFilterType::BandStop(
                    low_band
                        .parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*low_band)))?,
                    high_band
                        .parse::<f64>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*high_band)))?,
                );
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.apply_fft_filter(&current_chart_borrow, filter),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
            ChartViewCommands::HaarWaveletTransform => {
                let Some(current_chart) = self.state.current_chart() else {
                    return Err(CommandError::NoChart.into());
                };
                let current_chart_borrow = current_chart.borrow();
                let chart = ChartModel::new(
                    self.service.haar_wavelet_transform(&current_chart_borrow),
                    current_chart_borrow.metadata.chart_display_type,
                    current_chart_borrow.sample_rate,
                    &current_chart_borrow.metadata.title,
                    Some(&current_chart_borrow.metadata.description), // TODO: Change description
                );
                state_borrow.add_chart(chart);
            }
        };
        state_borrow.set_command(None);
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let Some(current_dataset) = &self.app_state.borrow().get_current_chart() else {
            self.state.set_current_chart(None);
            return;
        };
        self.state.set_current_chart(Some(current_dataset.clone()));
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
                self.canvas_generate_labels(context, self.state.canvas_style().canvas_steps);
                self.canvas_generate_grid(context, self.state.canvas_style().canvas_steps);
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
                self.state.canvas_style().canvas_color,
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
                self.state.canvas_style().canvas_color,
            ));
        });
    }
}
