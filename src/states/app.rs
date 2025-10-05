pub struct ApplicationState {
    is_running: bool,
    is_input_mode: bool,
    is_static_mode: bool,

    workspace_size: u16,
    version_component_size: u16,
    help_component_size: u16,

    graphic_workspace_size: u16,
    file_explorer_size: u16,

    command: Option<String>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            is_running: true,
            is_input_mode: false,
            is_static_mode: false,
            file_explorer_size: 15,
            graphic_workspace_size: 85,
            workspace_size: 100,
            version_component_size: 0,
            help_component_size: 0,
            command: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn is_input_mode(&self) -> bool {
        self.is_input_mode
    }

    pub fn file_explorer_size(&self) -> u16 {
        self.file_explorer_size
    }

    pub fn graphic_workspace_size(&self) -> u16 {
        self.graphic_workspace_size
    }

    pub fn workspace_size(&self) -> u16 {
        self.workspace_size
    }

    pub fn version_component_size(&self) -> u16 {
        self.version_component_size
    }

    pub fn help_component_size(&self) -> u16 {
        self.help_component_size
    }

    pub fn command(&self) -> Option<String> {
        self.command.clone()
    }

    pub fn set_command(&mut self, command: Option<String>) {
        self.command = command;
    }

    pub fn to_input_mode(&mut self) {
        self.is_input_mode = true;
        self.is_static_mode = false;
    }

    pub fn to_static_mode(&mut self) {
        self.is_input_mode = false;
        self.is_static_mode = true;
        self.version_component_size = 0;
        self.help_component_size = 0;
        self.workspace_size = 100;
    }

    pub fn show_version(&mut self) {
        self.to_static_mode();
        self.workspace_size = 0;
        self.version_component_size = 100;
    }

    pub fn show_help(&mut self) {
        self.to_static_mode();
        self.workspace_size = 0;
        self.help_component_size = 100;
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn change_file_explorer_visibility(&mut self) {
        if self.file_explorer_size == 0 {
            self.file_explorer_size = 15;
            self.graphic_workspace_size = 85;
        } else {
            self.file_explorer_size = 0;
            self.graphic_workspace_size = 100;
        }
    }
}
