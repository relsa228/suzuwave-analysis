use ratatui::style::Color;

use crate::shared::constants::general::DEFAULT_COLOR;

pub struct ChartViewStyle {
    pub canvas_color: Color,
    pub canvas_steps: u32,
}

impl ChartViewStyle {
    pub fn new() -> Self {
        Self {
            canvas_color: DEFAULT_COLOR,
            canvas_steps: 17,
        }
    }
}
