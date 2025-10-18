use ratatui::style::Color;

use crate::shared::constants::general::DEFAULT_COLOR;

pub struct ChartViewStyle {
    canvas_color: Color,
    canvas_steps: u32,
}

impl ChartViewStyle {
    pub fn new() -> Self {
        Self {
            canvas_color: DEFAULT_COLOR,
            canvas_steps: 17,
        }
    }

    pub fn canvas_color(&self) -> Color {
        self.canvas_color
    }

    pub fn canvas_steps(&self) -> u32 {
        self.canvas_steps
    }
}
