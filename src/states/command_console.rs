use anyhow::Error;
use ratatui::style::Color;

use crate::{
    models::command_console::style::CommandConsoleStyle,
    shared::constants::command::DEFAULT_COMMAND,
};

pub struct CommandConsoleState {
    input: String,
    is_input_error: bool,
    style: CommandConsoleStyle,
}

impl CommandConsoleState {
    pub fn new() -> Self {
        Self {
            input: String::from(DEFAULT_COMMAND),
            is_input_error: false,
            style: CommandConsoleStyle::new(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn flush_input(&mut self) {
        self.input.clear();
        self.input.push_str(DEFAULT_COMMAND);
    }

    pub fn input_and_flush(&mut self) -> String {
        let input = self.input.clone();
        self.flush_input();
        input
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn style_as_mut(&mut self) -> &mut CommandConsoleStyle {
        &mut self.style
    }

    pub fn style(&self) -> &CommandConsoleStyle {
        &self.style
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

    pub fn is_input_error(&self) -> bool {
        self.is_input_error
    }
}
