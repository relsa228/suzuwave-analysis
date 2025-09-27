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

use crate::{models::graphic_view::point::Point, states::graphic_view::GraphicViewState};

pub struct GraphicViewComponent {
    state: GraphicViewState,
}

impl GraphicViewComponent {
    pub fn new(plot: Vec<Point>) -> Self {
        Self {
            state: GraphicViewState::new(plot),
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.state.plot_move(true);
            }
            KeyCode::Right => {
                self.state.plot_move(false);
            }
            KeyCode::Up => {
                self.state.plot_scale(true);
            }
            KeyCode::Down => {
                self.state.plot_scale(false);
            }
            _ => {}
        }
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
