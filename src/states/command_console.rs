use anyhow::Error;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};

use crate::{
    models::command_console::style::CommandConsoleStyle,
    shared::constants::{
        command::{DEFAULT_COMMAND_PREFIX, DEFAULT_CURSOR},
        general::DEFAULT_COLOR,
    },
};

pub struct CommandConsoleState {
    input: String,
    is_input_error: bool,
    style: CommandConsoleStyle,
    cursor_position: usize,
    command_history: Vec<String>,
}

impl CommandConsoleState {
    pub fn new() -> Self {
        Self {
            input: String::from(DEFAULT_COMMAND_PREFIX),
            is_input_error: false,
            style: CommandConsoleStyle::new(),
            cursor_position: 1,
            command_history: Vec::new(),
        }
    }

    pub fn cursor_move(&mut self, left: bool) {
        if left {
            if self.cursor_position > 1 {
                self.cursor_position -= 1;
            }
        } else {
            if self.cursor_position < self.input.len() {
                self.cursor_position += 1;
            }
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn render_input(&self) -> Line<'_> {
        if self.is_input_error {
            Line::from(self.input.clone())
        } else {
            let mut iter = self.input.chars();
            let left: String = iter.by_ref().take(self.cursor_position).collect();
            let right: String = iter.collect();

            Line::from(vec![
                Span::raw(left),
                Span::styled(DEFAULT_CURSOR, Style::default().fg(Color::LightGreen)),
                Span::raw(right),
            ])
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn push_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 1 {
            self.input.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn flush_input(&mut self) {
        self.input.clear();
        self.cursor_position = 1;
        self.input.push_str(DEFAULT_COMMAND_PREFIX);
    }

    pub fn input_and_flush(&mut self) -> String {
        let input = self.input.clone();
        self.flush_input();
        input
    }

    pub fn is_input_error(&self) -> bool {
        self.is_input_error
    }

    pub fn set_error(&mut self, error: Error) {
        self.is_input_error = true;
        self.style_as_mut().set_input_color(Color::Red);
        self.style_as_mut().set_borders_color(Color::Red);
        self.set_input(error.to_string());
    }

    pub fn clear_error(&mut self) {
        self.is_input_error = false;
        self.style_as_mut().set_input_color(Color::Green);
        self.style_as_mut().set_borders_color(Color::Green);
    }

    pub fn style(&self) -> &CommandConsoleStyle {
        &self.style
    }

    pub fn style_as_mut(&mut self) -> &mut CommandConsoleStyle {
        &mut self.style
    }
}
