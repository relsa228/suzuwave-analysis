use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|f| {
                let size = f.area();
                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
                    .split(size);

                let workspace_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(2), Constraint::Min(5), Constraint::Max(3)])
                    .split(main_chunks[1]);

                f.render_widget(
                    Block::default()
                        .title("File Explorer")
                        .borders(Borders::ALL),
                    main_chunks[0],
                );

                f.render_widget(
                    Block::default().title("View tabs").borders(Borders::ALL),
                    workspace_chunks[0],
                );
                f.render_widget(
                    Block::default().title("Graphic View").borders(Borders::ALL),
                    workspace_chunks[1],
                );
                f.render_widget(
                    Block::default().title("Command").borders(Borders::ALL),
                    workspace_chunks[2],
                );
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
