use std::{cell::RefCell, rc::Rc};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};

use crate::{
    components::{command_console::CommandConsoleComponent, graphic_view::GraphicViewComponent},
    models::graphic_view::point::Point,
    states::app::ApplicationState,
};

pub struct App {
    application_state: Rc<RefCell<ApplicationState>>,

    command_console: CommandConsoleComponent,
    graphic_widget: GraphicViewComponent,
}

impl App {
    pub fn new(data: Vec<Point>) -> Self {
        let application_state = Rc::new(RefCell::new(ApplicationState::new()));
        Self {
            application_state: application_state.clone(),
            command_console: CommandConsoleComponent::new(application_state),
            graphic_widget: GraphicViewComponent::new(data),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.application_state.borrow().running() {
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

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.graphic_widget.handle_key_events(key);
                if self.application_state.borrow().input_mode() {
                    self.command_console.handle_key_events(key);
                } else {
                    self.handle_key_events(key);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
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
            // Add other key handlers here.
            _ => {}
        }
    }
}
