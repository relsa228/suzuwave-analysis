pub struct ApplicationState {
    is_running: bool,
    is_input_mode: bool,
    is_static_mode: bool,

    file_explorer_size: u16,
    workspace_size: u16,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            is_running: true,
            is_input_mode: false,
            is_static_mode: false,
            file_explorer_size: 15,
            workspace_size: 85,
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

    pub fn running(&self) -> bool {
        self.is_running
    }

    pub fn input_mode(&self) -> bool {
        self.is_input_mode
    }

    pub fn static_mode(&self) -> bool {
        self.is_static_mode
    }

    pub fn file_explorer_size(&self) -> u16 {
        self.file_explorer_size
    }

    pub fn workspace_size(&self) -> u16 {
        self.workspace_size
    }
}
