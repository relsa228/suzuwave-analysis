use ratatui::{
    style::Color,
    widgets::{BorderType, Borders},
};

pub struct CommandConsoleStyle {
    pub borders: Borders,
    pub borders_type: BorderType,
    pub input_color: Color,
    pub border_color: Color,
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
}
