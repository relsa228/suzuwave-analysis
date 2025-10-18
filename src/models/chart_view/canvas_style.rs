use ratatui::{
    style::Color,
    widgets::canvas::{Map, MapResolution},
};

use crate::shared::constants::general::DEFAULT_COLOR;

pub struct ChartViewStyle {
    canvas_color: Color,
    canvas_steps: u32,
    map: Map,
}

impl ChartViewStyle {
    pub fn new() -> Self {
        Self {
            canvas_color: DEFAULT_COLOR,
            canvas_steps: 17,
            map: Map {
                resolution: MapResolution::Low,
                color: DEFAULT_COLOR,
            },
        }
    }

    pub fn canvas_color(&self) -> Color {
        self.canvas_color
    }

    pub fn canvas_steps(&self) -> u32 {
        self.canvas_steps
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
