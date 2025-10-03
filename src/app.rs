use anyhow::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};
use std::{cell::RefCell, path::Path, rc::Rc, str::FromStr};

use crate::{
    components::{command_console::CommandConsoleComponent, graphic_view::GraphicViewComponent},
    shared::{
        commands::general::GeneralCommands, constants::command::DEFAULT_COMMAND,
        errors::commands::CommandError,
    },
    states::app::ApplicationState,
};

pub struct App {
    application_state: Rc<RefCell<ApplicationState>>,
    command_console: CommandConsoleComponent,
    graphic_widget: GraphicViewComponent,
}

impl App {
    pub fn new(initial_signal_file_path: Option<&Path>) -> Self {
        let application_state = Rc::new(RefCell::new(ApplicationState::new()));
        Self {
            application_state: application_state.clone(),
            command_console: CommandConsoleComponent::new(),
            graphic_widget: GraphicViewComponent::new(initial_signal_file_path),
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
                        Constraint::Percentage(
                            self.application_state.borrow().file_explorer_size(),
                        ),
                        Constraint::Percentage(self.application_state.borrow().workspace_size()),
                    ])
                    .split(size);

                let workspace_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Max(3)])
                    .split(main_chunks[1]);

                f.render_widget(
                    Block::default()
                        .title("File Explorer")
                        .borders(Borders::ALL),
                    main_chunks[0],
                );

                self.graphic_widget.render(f, workspace_chunks[0]);
                self.command_console.render(f, workspace_chunks[1]);
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
        let is_file_explorer_visible = app_state.is_file_explorer_visible();
        let cmd = app_state.command().clone();
        if let Some(cmd) = cmd {
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() || args[0] == DEFAULT_COMMAND {
                return Err(CommandError::EmptyCommand.into());
            }
            if let Ok(command) = GeneralCommands::from_str(args[0]) {
                match command {
                    GeneralCommands::About => todo!(),
                    GeneralCommands::Help => todo!(),
                    GeneralCommands::OpenCloseFileExplorer => {
                        app_state.change_file_explorer_visibility(!is_file_explorer_visible)
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
            _ => {}
        }
    }
}
