use std::{cell::RefCell, rc::Rc, sync::Arc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::states::{app::ApplicationState, command_console::CommandConsoleState};

pub struct CommandConsoleComponent {
    state: CommandConsoleState,
    application_state: Rc<RefCell<ApplicationState>>,
}

impl CommandConsoleComponent {
    pub fn new(application_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self {
            state: CommandConsoleState::new(),
            application_state,
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char(':')) => {
                self.state.flush_input();
                self.state.push_char(':');
            }
            (_, KeyCode::Char(c)) => {
                if self.application_state.borrow().input_mode() {
                    self.state.push_char(c);
                }
            }
            (_, KeyCode::Backspace) => {
                self.state.pop_char();
            }
            (_, KeyCode::Enter) => {
                let command = self.state.input_and_flush();
                self.state.push_char(':');
                // Send the command
            }
            (_, KeyCode::Esc) => {
                self.application_state.borrow_mut().to_static_mode();
            }
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let input_widget = Paragraph::new(self.state.input())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_widget, rect);
    }
}
