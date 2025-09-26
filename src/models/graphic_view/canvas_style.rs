use ratatui::{
    style::Color,
    widgets::canvas::{Map, MapResolution},
};

pub struct GraphicViewStyle {
    canvas_color: Color,
    canvas_steps: u32,
    map: Map,
}

impl GraphicViewStyle {
    pub fn new() -> Self {
        Self {
            canvas_color: Color::LightYellow,
            canvas_steps: 17,
            map: Map {
                resolution: MapResolution::Low,
                color: Color::LightYellow,
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
