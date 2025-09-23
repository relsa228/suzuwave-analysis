pub struct CommandConsoleState {
    input: String,
}

impl CommandConsoleState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub fn flush_input(&mut self) {
        self.input.clear();
    }

    pub fn input_and_flush(&mut self) -> String {
        let input = self.input.clone();
        self.flush_input();
        input
    }

    pub fn input(&self) -> &str {
        &self.input
    }
}
