use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use std::{cell::RefCell, rc::Rc};

use crate::states::{app::ApplicationState, command_console::CommandConsoleState};

pub struct CommandConsoleComponent {
    state: CommandConsoleState,
}

impl CommandConsoleComponent {
    pub fn new() -> Self {
        Self {
            state: CommandConsoleState::new(),
        }
    }

    pub fn handle_key_events(
        &mut self,
        key: KeyEvent,
        application_state: Rc<RefCell<ApplicationState>>,
    ) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char(':')) => {
                self.state.flush_input();
            }
            (_, KeyCode::Char(c)) => {
                if application_state.borrow().is_input_mode() {
                    self.state.push_char(c);
                }
            }
            (_, KeyCode::Backspace) => {
                self.state.pop_char();
            }
            (_, KeyCode::Enter) => {
                if !self.state.is_input_error() {
                    application_state
                        .borrow_mut()
                        .set_command(Some(self.state.input_and_flush()));
                } else {
                    self.state.clear_error();
                    self.state.flush_input();
                }
            }
            (_, KeyCode::Esc) => {
                application_state.borrow_mut().to_static_mode();
                self.disable_input_mode();
            }
            _ => {}
        }
    }

    pub fn to_input_mode(&mut self) {
        self.state.style_as_mut().set_borders_color(Color::Green);
        self.state.style_as_mut().set_input_color(Color::Green);
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let input_widget = Paragraph::new(self.state.input())
            .style(Style::default().fg(self.state.style().input_color()))
            .block(
                Block::default()
                    .borders(self.state.style().borders())
                    .border_type(self.state.style().borders_type())
                    .border_style(Style::default().fg(self.state.style().border_color())),
            );
        f.render_widget(input_widget, rect);
    }

    pub fn disable_input_mode(&mut self) {
        self.state.flush_input();
        self.state
            .style_as_mut()
            .set_borders_color(Color::LightGreen);
        self.state.style_as_mut().set_input_color(Color::LightGreen);
    }

    pub fn set_error(&mut self, error: Error) {
        self.state.set_error(error);
    }
}
