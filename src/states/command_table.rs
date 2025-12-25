use ratatui::{
    layout::Constraint,
    style::{Color, Style},
};

use crate::shared::constants::general::DEFAULT_COLOR;

pub struct CommandTableState {
    table_widths: [Constraint; 3],
    headers_style: Style,
    data_row_style: Style,
    block_style: Style,
}

impl Default for CommandTableState {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandTableState {
    pub fn new() -> Self {
        Self {
            table_widths: [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ],
            headers_style: Style::default().bg(Color::Yellow).fg(Color::Black),
            data_row_style: Style::default().fg(DEFAULT_COLOR),
            block_style: Style::default().fg(Color::Yellow),
        }
    }

    pub fn table_widths(&self) -> &[Constraint; 3] {
        &self.table_widths
    }

    pub fn headers_style(&self) -> Style {
        self.headers_style
    }

    pub fn data_row_style(&self) -> Style {
        self.data_row_style
    }

    pub fn block_style(&self) -> Style {
        self.block_style
    }
}
