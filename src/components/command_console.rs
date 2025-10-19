use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use std::{cell::RefCell, rc::Rc};

use crate::{
    components::component::Component,
    states::{
        app::{ApplicationMode, ApplicationState},
        command_console::CommandConsoleState,
    },
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

    /// Go to the input mode.
    ///
    /// This function sets the component's state to input mode.
    /// In this mode, the user can input and edit commands.
    fn enable_input_mode(&mut self) {
        let mut state = self.state.borrow_mut();
        if self.app_state.borrow().mode() == ApplicationMode::Input {
            state.style_as_mut().border_color = Color::Green;
            state.style_as_mut().input_color = Color::Green;
        } else {
            state.style_as_mut().border_color = Color::Red;
            state.style_as_mut().input_color = Color::Red;
        }
    }

    /// Go to the normal mode.
    ///
    /// This function sets the component's state to normal mode.
    /// In this mode, the user can only view the commands.
    fn disable_input_mode(&mut self) {
        let mut state = self.state.borrow_mut();
        state.style_as_mut().border_color = Color::LightGreen;
        state.style_as_mut().input_color = Color::LightGreen;
    }

    /// Go to the error mode.
    ///
    /// In this mode, the user can only view the error message and flush it.
    fn set_error(&self) {
        let mut state = self.app_state.borrow_mut();
        if let Some(error) = state.error() {
            let error_str = error.to_string();
            self.state.borrow_mut().set_error(error_str);
            state.set_error(None);
        }
    }
}

impl Component for CommandConsoleComponent {
    fn handle_key_event(&mut self, key: KeyEvent) {
        let mut state = self.state.borrow_mut();
        let is_error = self.app_state.borrow().mode() == ApplicationMode::Error;
        match key.code {
            KeyCode::Char(':') => {
                state.flush_input();
                self.app_state.borrow_mut().to_input_mode();
            }
            KeyCode::Char(c) => {
                if !is_error {
                    state.push_char(c);
                }
            }
            KeyCode::Backspace => {
                if !is_error {
                    state.remove_char();
                }
            }
            KeyCode::Left => {
                if !is_error {
                    state.input_cursor_move(true);
                }
            }
            KeyCode::Right => {
                if !is_error {
                    state.input_cursor_move(false);
                }
            }
            KeyCode::Up => {
                state.clear_error();
                state.move_history_cache_cursor(true);
            }
            KeyCode::Down => {
                state.clear_error();
                state.move_history_cache_cursor(false);
            }
            KeyCode::Enter => {
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
            KeyCode::Esc => {
                self.app_state.borrow_mut().to_static_mode();
                drop(state);
                self.disable_input_mode();
            }
            _ => {}
        }
    }

    fn render(&mut self, f: &mut Frame, rect: Rect) {
        let mode = self.app_state.borrow().mode();
        match mode {
            ApplicationMode::Input => {
                self.enable_input_mode();
            }
            ApplicationMode::Static | ApplicationMode::Explorer => {
                self.disable_input_mode();
            }
            ApplicationMode::Error => {
                self.set_error();
            }
        }
        let state = self.state.borrow_mut();
        let input_widget = Paragraph::new(state.render_input())
            .style(Style::default().fg(state.style().input_color))
            .block(
                Block::default()
                    .borders(state.style().borders)
                    .border_type(state.style().borders_type)
                    .border_style(Style::default().fg(state.style().border_color)),
            );

        f.render_widget(input_widget, rect);
    }

    fn update_from_state(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
