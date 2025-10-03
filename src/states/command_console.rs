use crate::models::command_console::style::CommandConsoleStyle;

pub struct CommandConsoleState {
    input: String,
    style: CommandConsoleStyle,
}

impl CommandConsoleState {
    pub fn new() -> Self {
        Self {
            input: String::from(":"),
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
        self.input.push(':');
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
}
