use ratatui::{
    layout::Alignment,
    style::{Color, Style},
};

use crate::shared::constants::general::DEFAULT_COLOR;

pub struct AboutState {
    block_style: Style,
    text_style: Style,
    alignment: Alignment,
}

impl AboutState {
    pub fn new() -> Self {
        Self {
            block_style: Style::default().fg(DEFAULT_COLOR),
            text_style: Style::default().fg(Color::Yellow),
            alignment: Alignment::Center,
        }
    }

    pub fn block_style(&self) -> Style {
        self.block_style
    }

    pub fn text_style(&self) -> Style {
        self.text_style
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}
