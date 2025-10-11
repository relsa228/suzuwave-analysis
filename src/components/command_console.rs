use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use std::{cell::RefCell, rc::Rc};

use crate::states::{
    app::{ApplicationMode, ApplicationState},
    command_console::CommandConsoleState,
};

pub struct CommandConsoleComponent {
    state: Rc<RefCell<CommandConsoleState>>,
    app_state: Rc<RefCell<ApplicationState>>,
}

impl CommandConsoleComponent {
    pub fn new(app_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self {
            state: Rc::new(RefCell::new(CommandConsoleState::new())),
            app_state,
        }
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        let mut state = self.state.borrow_mut();
        let is_error = self.app_state.borrow().mode() == ApplicationMode::Error;
        match (key.modifiers, key.code) {
            (_, KeyCode::Char(':')) => {
                state.flush_input();
                self.app_state.borrow_mut().to_input_mode();
            }
            (_, KeyCode::Char(c)) => {
                if !is_error {
                    state.push_char(c);
                }
            }
            (_, KeyCode::Backspace) => {
                if !is_error {
                    state.remove_char();
                }
            }
            (_, KeyCode::Left) => {
                if !is_error {
                    state.cursor_move(true);
                }
            }
            (_, KeyCode::Right) => {
                if !is_error {
                    state.cursor_move(false);
                }
            }
            (_, KeyCode::Up) => {
                state.clear_error();
                state.move_history_cursor(true);
            }
            (_, KeyCode::Down) => {
                state.clear_error();
                state.move_history_cursor(false);
            }
            (_, KeyCode::Enter) => {
                if !is_error {
                    self.app_state
                        .borrow_mut()
                        .set_command(Some(state.input_and_flush()));
                } else {
                    state.clear_error();
                    state.flush_input();
                    self.app_state.borrow_mut().to_input_mode();
                }
            }
            (_, KeyCode::Esc) => {
                self.app_state.borrow_mut().to_static_mode();
                drop(state);
                self.disable_input_mode();
            }
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let mode = self.app_state.borrow().mode();
        match mode {
            ApplicationMode::Input => {
                self.enable_input_mode();
            }
            ApplicationMode::Static => {
                self.disable_input_mode();
            }
            ApplicationMode::Error => {
                self.set_error();
            }
        }
        let state = self.state.borrow_mut();
        let input_widget = Paragraph::new(state.render_input())
            .style(Style::default().fg(state.style().input_color()))
            .block(
                Block::default()
                    .borders(state.style().borders())
                    .border_type(state.style().borders_type())
                    .border_style(Style::default().fg(state.style().border_color())),
            );

        f.render_widget(input_widget, rect);
    }

    fn enable_input_mode(&mut self) {
        let mut state = self.state.borrow_mut();
        if self.app_state.borrow().mode() == ApplicationMode::Input {
            state.style_as_mut().set_borders_color(Color::Green);
            state.style_as_mut().set_input_color(Color::Green);
        } else {
            state.style_as_mut().set_borders_color(Color::Red);
            state.style_as_mut().set_input_color(Color::Red);
        }
    }

    fn disable_input_mode(&mut self) {
        let mut state = self.state.borrow_mut();
        state.style_as_mut().set_borders_color(Color::LightGreen);
        state.style_as_mut().set_input_color(Color::LightGreen);
    }

    fn set_error(&self) {
        let mut state = self.app_state.borrow_mut();
        if let Some(error) = state.error() {
            let error_str = error.to_string();
            self.state.borrow_mut().set_error(error_str);
            state.set_error(None);
        }
    }
}
