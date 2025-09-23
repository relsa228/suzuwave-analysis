pub struct ApplicationState {
    is_running: bool,
    is_input_mode: bool,
    is_static_mode: bool,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            is_running: true,
            is_input_mode: false,
            is_static_mode: false,
        }
    }

    pub fn to_input_mode(&mut self) {
        self.is_input_mode = true;
        self.is_static_mode = false;
    }

    pub fn to_static_mode(&mut self) {
        self.is_input_mode = false;
        self.is_static_mode = true;
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn running(&self) -> bool {
        self.is_running
    }

    pub fn input_mode(&self) -> bool {
        self.is_input_mode
    }

    pub fn static_mode(&self) -> bool {
        self.is_static_mode
    }
}
