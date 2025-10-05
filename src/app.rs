use anyhow::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders},
};
use std::{cell::RefCell, path::PathBuf, rc::Rc, str::FromStr};

use crate::{
    components::{
        about::AboutComponent, command_console::CommandConsoleComponent,
        command_table::CommandTable, graphic_view::GraphicViewComponent,
    },
    shared::{
        commands::general::GeneralCommands,
        constants::{command::DEFAULT_COMMAND_PREFIX, general::DEFAULT_COLOR},
        errors::commands::CommandError,
    },
    states::app::ApplicationState,
};

pub struct App {
    application_state: Rc<RefCell<ApplicationState>>,
    command_console: CommandConsoleComponent,
    graphic_widget: GraphicViewComponent,
    version_component: AboutComponent,
    help_component: CommandTable,
}

impl App {
    pub fn new(initial_signal_file_path: Option<PathBuf>) -> Self {
        let application_state = Rc::new(RefCell::new(ApplicationState::new()));
        Self {
            application_state: application_state.clone(),
            command_console: CommandConsoleComponent::new(),
            graphic_widget: GraphicViewComponent::new(initial_signal_file_path),
            version_component: AboutComponent::new(),
            help_component: CommandTable::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.application_state.borrow().is_running() {
            terminal.draw(|f| {
                let size = f.area();
                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(self.application_state.borrow().workspace_size()),
                        Constraint::Percentage(
                            self.application_state.borrow().version_component_size(),
                        ),
                        Constraint::Percentage(
                            self.application_state.borrow().help_component_size(),
                        ),
                    ])
                    .split(size);

                let workspace_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(
                            self.application_state.borrow().file_explorer_size(),
                        ),
                        Constraint::Percentage(
                            self.application_state.borrow().graphic_workspace_size(),
                        ),
                    ])
                    .split(main_chunks[0]);

                let graphic_workspace = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Max(3)])
                    .split(workspace_chunks[1]);

                f.render_widget(
                    Block::default()
                        .title("File Explorer")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(DEFAULT_COLOR)),
                    workspace_chunks[0],
                );

                self.graphic_widget.render(f, graphic_workspace[0]);
                self.command_console.render(f, graphic_workspace[1]);
                self.version_component.render(f, main_chunks[1]);
                self.help_component.render(f, main_chunks[2]);
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if self.application_state.borrow().is_input_mode() {
                    self.command_console
                        .handle_key_events(key, self.application_state.clone());

                    let mut error: Option<Error> = None;
                    if let Err(err) = self
                        .graphic_widget
                        .update_from_state(self.application_state.clone())
                    {
                        error = Some(err);
                    };
                    if let Err(err) = self.update_from_state() {
                        error = Some(err);
                    };

                    if let Some(error) = error {
                        self.command_console.set_error(error);
                    } else if let Some(command) = self.application_state.borrow().command() {
                        self.command_console
                            .set_error(CommandError::CommandSyntax(command).into());
                    }
                    self.application_state.borrow_mut().set_command(None);
                } else {
                    self.graphic_widget.handle_key_events(key);
                    self.handle_key_events(key);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn update_from_state(&mut self) -> anyhow::Result<()> {
        let mut app_state = self.application_state.borrow_mut();
        let cmd = app_state.command().clone();
        if let Some(cmd) = cmd {
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
                return Err(CommandError::EmptyCommand.into());
            }
            if let Ok(command) = GeneralCommands::from_str(args[0]) {
                match command {
                    GeneralCommands::About => {
                        app_state.show_version();
                    }
                    GeneralCommands::Help => {
                        app_state.show_help();
                    }
                    GeneralCommands::OpenCloseFileExplorer => {
                        app_state.change_file_explorer_visibility()
                    }
                    GeneralCommands::OpenSettings => unimplemented!(),
                    GeneralCommands::Quit => app_state.quit(),
                }
                app_state.set_command(None);
            }
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.application_state.borrow_mut().quit()
            }
            (_, KeyCode::Char('i') | KeyCode::Char('I')) => {
                self.command_console.to_input_mode();
                self.application_state.borrow_mut().to_input_mode()
            }
            (_, KeyCode::Esc) => {
                self.application_state.borrow_mut().to_static_mode();
                self.command_console.disable_input_mode();
            }
            _ => {}
        }
    }
}
