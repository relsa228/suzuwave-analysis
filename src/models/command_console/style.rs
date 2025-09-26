use ratatui::{
    style::Color,
    widgets::{BorderType, Borders},
};

pub struct CommandConsoleStyle {
    borders: Borders,
    borders_type: BorderType,
    input_color: Color,
    border_color: Color,
}

impl CommandConsoleStyle {
    pub fn new() -> Self {
        Self {
            borders: Borders::ALL,
            borders_type: BorderType::Rounded,
            input_color: Color::LightGreen,
            border_color: Color::LightGreen,
        }
    }

    pub fn borders(&self) -> Borders {
        self.borders
    }

    pub fn borders_type(&self) -> BorderType {
        self.borders_type
    }

    pub fn input_color(&self) -> Color {
        self.input_color
    }

    pub fn border_color(&self) -> Color {
        self.border_color
    }

    pub fn set_borders_color(&mut self, border_color: Color) {
        self.border_color = border_color;
    }

    pub fn set_input_color(&mut self, input_color: Color) {
        self.input_color = input_color;
    }
}
