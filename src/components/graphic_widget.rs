use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

use crate::states::graphic_widget::GraphicWidgetState;

pub struct GraphicWidgetComponent {
    state: GraphicWidgetState,
}

impl GraphicWidgetComponent {
    pub fn new() -> Self {
        Self {
            state: GraphicWidgetState::new(),
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.state.x_min_add(-1.0);
                self.state.x_max_add(-1.0);
            }
            KeyCode::Right => {
                self.state.x_min_add(1.0);
                self.state.x_max_add(1.0);
            }
            KeyCode::Up => {
                let center = (self.state.x_min() + self.state.x_max()) / 2.0;
                let half = (self.state.x_max() - self.state.x_min()) / 2.0 * 0.8;
                self.state.set_x_min(center - half);
                self.state.set_x_max(center + half);
            }
            KeyCode::Down => {
                let center = (self.state.x_min() + self.state.x_max()) / 2.0;
                let half = (self.state.x_max() - self.state.x_min()) / 2.0 * 1.2;
                self.state.set_x_min(center - half);
                self.state.set_x_max(center + half);
            }
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let current_dataset = &self.state.current_dataset();
        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(GraphType::Line)
                .data(current_dataset),
        ];

        let chart = Chart::new(datasets)
            .block(Block::default().title("Chart").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .title("X")
                    .style(Style::default().fg(Color::Red))
                    .bounds([self.state.x_min(), self.state.x_max()]),
            )
            .y_axis(
                Axis::default()
                    .title("Y")
                    .style(Style::default().fg(Color::Red))
                    .bounds([self.state.y_min(), self.state.y_max()]),
            );
        f.render_widget(chart, rect);
    }
}
