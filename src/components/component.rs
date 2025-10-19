use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

pub trait Component {
    /// Render the component.
    ///
    /// This method is responsible for rendering the component on the screen.
    ///
    /// ---
    ///
    /// * `f`: The frame to render the component on.
    /// * `rect`: The rectangle to render the component in.
    fn render(&mut self, f: &mut Frame, rect: Rect);

    /// Handle a key event.
    ///
    /// This method is responsible for handling keyboard key events.
    ///
    /// ---
    ///
    /// * `key`: The key event to handle.
    fn handle_key_event(&mut self, key: KeyEvent);

    /// Update the component from the state.
    ///
    /// This method is responsible for updating the component
    /// from the general app state.
    fn update_from_state(&mut self) -> Result<()>;
}
