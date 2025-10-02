pub struct ApplicationState {
    pub is_running: bool,
    pub is_input_mode: bool,
    pub is_static_mode: bool,

    pub file_explorer_size: u16,
    pub workspace_size: u16,

    pub command: Option<String>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            is_running: true,
            is_input_mode: false,
            is_static_mode: false,
            file_explorer_size: 15,
            workspace_size: 85,
            command: None,
        }
    }

    pub fn to_input_mode(&mut self) {
        self.is_input_mode = true;
        self.is_static_mode = false;
        self.file_explorer_size = 0;
        self.workspace_size = 100;
    }

    pub fn to_static_mode(&mut self) {
        self.is_input_mode = false;
        self.is_static_mode = true;
        self.file_explorer_size = 15;
        self.workspace_size = 85;
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
