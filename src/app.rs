use crate::{
    components::{
        about::AboutComponent, chart_explorer::ChartExplorerComponent,
        chart_view::ChartViewComponent, command_console::CommandConsoleComponent,
        command_table::CommandTableComponent, component::Component,
    },
    shared::{
        commands::general::GeneralCommands,
        constants::{
            command::DEFAULT_COMMAND_PREFIX,
            general::{
                EXPLORER_KEY_1, EXPLORER_KEY_2, INPUT_KEY_1, INPUT_KEY_2, QUIT_KEY_1, QUIT_KEY_2,
            },
        },
        errors::commands::CommandError,
    },
    states::app::{ApplicationMode, ApplicationState},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
};
use std::{cell::RefCell, path::PathBuf, rc::Rc, str::FromStr};

pub struct App {
    application_state: Rc<RefCell<ApplicationState>>,
    command_console: CommandConsoleComponent,
    chart_view_widget: ChartViewComponent,
    chart_explorer_widget: ChartExplorerComponent,
    version_component: AboutComponent,
    help_component: CommandTableComponent,
}

impl App {
    pub fn new(initial_signal_file_path: Option<PathBuf>) -> Self {
        let application_state = Rc::new(RefCell::new(ApplicationState::new()));
        Self {
            application_state: application_state.clone(),
            command_console: CommandConsoleComponent::new(application_state.clone()),
            chart_view_widget: ChartViewComponent::new(application_state.clone()),
            chart_explorer_widget: ChartExplorerComponent::new(
                initial_signal_file_path,
                application_state.clone(),
            ),
            version_component: AboutComponent::new(),
            help_component: CommandTableComponent::new(),
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
                            self.application_state.borrow().chart_workspace_size(),
                        ),
                    ])
                    .split(main_chunks[0]);

                let chart_workspace = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Max(3)])
                    .split(workspace_chunks[1]);

                self.chart_explorer_widget.render(f, workspace_chunks[0]);
                self.chart_view_widget.render(f, chart_workspace[0]);
                self.command_console.render(f, chart_workspace[1]);
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
                let mode = self.application_state.borrow().mode();
                match mode {
                    ApplicationMode::Explorer => {
                        self.chart_explorer_widget.handle_key_event(key);
                        self.handle_key_events(key);
                    }
                    ApplicationMode::Input | ApplicationMode::Error => {
                        self.command_console.handle_key_event(key);
                        let _ = self
                            .chart_view_widget
                            .update_from_state()
                            .is_err_and(|err| {
                                self.application_state.borrow_mut().set_error(Some(err))
                            });
                        let _ = self
                            .chart_explorer_widget
                            .update_from_state()
                            .is_err_and(|err| {
                                self.application_state.borrow_mut().set_error(Some(err))
                            });
                        let _ = self.update_from_state().is_err_and(|err| {
                            self.application_state.borrow_mut().set_error(Some(err))
                        });
                        let mut state = self.application_state.borrow_mut();
                        if state.error().is_none()
                            && let Some(command) = state.command()
                        {
                            state.set_error(Some(CommandError::CommandSyntax(command).into()));
                        }
                        state.set_command(None);
                    }
                    _ => {
                        self.chart_view_widget.handle_key_event(key);
                        self.handle_key_events(key);
                    }
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
        let Some(cmd) = app_state.command().clone() else {
            return Ok(());
        };
        let args = cmd.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
            return Err(CommandError::EmptyCommand.into());
        }
        let Ok(command) = GeneralCommands::from_str(args[0]) else {
            return Ok(());
        };
        match command {
            GeneralCommands::About => {
                app_state.show_version();
            }
            GeneralCommands::Help => {
                app_state.show_help();
            }
            GeneralCommands::OpenCloseChartsExplorer => {
                app_state.change_chart_explorer_visibility()
            }
            GeneralCommands::OpenSettings => unimplemented!(),
            GeneralCommands::Quit => app_state.quit(),
        }
        app_state.set_command(None);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char(QUIT_KEY_1) | KeyCode::Char(QUIT_KEY_2)) => {
                self.application_state.borrow_mut().quit()
            }
            (_, KeyCode::Char(INPUT_KEY_1) | KeyCode::Char(INPUT_KEY_2)) => {
                self.application_state.borrow_mut().to_input_mode()
            }
            (_, KeyCode::Char(EXPLORER_KEY_1) | KeyCode::Char(EXPLORER_KEY_2)) => {
                self.application_state.borrow_mut().to_explorer_mode()
            }
            (_, KeyCode::Esc) => {
                self.application_state.borrow_mut().to_static_mode();
            }
            _ => {}
        }
    }
}
