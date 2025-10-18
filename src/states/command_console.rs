use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    models::command_console::style::CommandConsoleStyle,
    shared::constants::command::{BUFFER_SIZE, DEFAULT_COMMAND_PREFIX, DEFAULT_CURSOR},
};

pub struct CommandConsoleState {
    input: String,
    is_input_error: bool,
    style: CommandConsoleStyle,
    cursor_position: usize,
    command_history: Vec<String>,
    history_cursor: usize,
}

impl CommandConsoleState {
    pub fn new() -> Self {
        Self {
            input: String::from(DEFAULT_COMMAND_PREFIX),
            is_input_error: false,
            style: CommandConsoleStyle::new(),
            cursor_position: 1,
            command_history: Vec::new(),
            history_cursor: 0,
        }
    }

    // Getters
    pub fn style(&self) -> &CommandConsoleStyle {
        &self.style
    }

    pub fn style_as_mut(&mut self) -> &mut CommandConsoleStyle {
        &mut self.style
    }

    // Commands history cache
    fn to_history_cache(&mut self) {
        if !self.input.is_empty() {
            self.command_history.push(self.input.clone());
        }
        if self.command_history.len() > BUFFER_SIZE {
            self.command_history.remove(0);
        }
        self.history_cursor = self.command_history.len();
    }

    pub fn move_history_cursor(&mut self, forward: bool) {
        self.history_cursor = if !forward {
            if self.history_cursor + 1 < self.command_history.len() {
                self.history_cursor + 1
            } else {
                0
            }
        } else {
            if self.history_cursor == 0 {
                self.command_history.len() - 1
            } else {
                self.history_cursor - 1
            }
        };
        self.input = self
            .command_history
            .get(self.history_cursor)
            .cloned()
            .unwrap_or_default();
        self.cursor_position = self.input.len();
    }

    // Input line && cursor management
    pub fn input(&self) -> &str {
        &self.input
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
        self.to_history_cache();
        self.flush_input();
        input
    }

    pub fn set_error(&mut self, error: String) {
        self.is_input_error = true;
        self.style_as_mut().input_color = Color::Red;
        self.style_as_mut().border_color = Color::Red;
        self.set_input(error);
    }

    pub fn clear_error(&mut self) {
        self.is_input_error = false;
        self.style_as_mut().input_color = Color::Green;
        self.style_as_mut().border_color = Color::Green;
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

    pub fn render_input(&self) -> Line<'_> {
        if self.is_input_error {
            return Line::from(self.input.clone());
        }
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
