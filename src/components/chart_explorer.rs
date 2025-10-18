use crate::{
    clients::{files::vibric::VibricReadingClient, traits::file_read_only::FileReadOnly},
    models::files::file_types::FileType,
    shared::{
        commands::chart_explorer::ChartExplorerCommands,
        constants::{chart_explorer::CHART_EXPLORER_WIDGET_NAME, command::DEFAULT_COMMAND_PREFIX},
        errors::{commands::CommandError, files::FileError},
    },
    states::app::{ApplicationMode, ApplicationState},
};
use anyhow::{Result, anyhow};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct ChartExplorerComponent {
    app_state: Rc<RefCell<ApplicationState>>,
    file_parsers: HashMap<FileType, Box<dyn FileReadOnly>>,
    list_state: ListState,
}

impl ChartExplorerComponent {
    pub fn new(
        initial_signal_file: Option<PathBuf>,
        app_state: Rc<RefCell<ApplicationState>>,
    ) -> Self {
        let mut file_parsers: HashMap<FileType, Box<dyn FileReadOnly>> = HashMap::new();
        file_parsers.insert(FileType::Vibric, Box::new(VibricReadingClient::new()));
        let instance = Self {
            file_parsers: file_parsers,
            app_state,
            list_state: ListState::default(),
        };
        if let Some(file) = initial_signal_file {
            let _ = instance.add_chart_from_file(file);
        }
        instance
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.app_state.borrow_mut().move_current_chart_backward(),
            KeyCode::Down => self.app_state.borrow_mut().move_current_chart_forward(),
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.app_state.borrow_mut().delete_current_chart()
            }
            _ => {}
        }
    }

    pub fn update_from_state(&mut self) -> Result<()> {
        let state_borrow = self.app_state.borrow();
        let cmd = if let Some(cmd) = state_borrow.command() {
            drop(state_borrow);
            cmd
        } else {
            return Ok(());
        };
        let args = cmd.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
            return Err(CommandError::EmptyCommand.into());
        }
        let command = if let Ok(command) = ChartExplorerCommands::from_str(args[0]) {
            command
        } else {
            self.app_state.borrow_mut().set_command(None);
            return Ok(());
        };
        match command {
            ChartExplorerCommands::CloseWorkingView => {
                self.app_state.borrow_mut().delete_current_chart();
            }
            ChartExplorerCommands::SwitchWorkingView => {
                let points_arg = if let Some(points_arg) = args.get(1) {
                    points_arg
                        .parse::<u32>()
                        .map_err(|_| CommandError::InvalidArguments(String::from(*points_arg)))?
                } else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                self.app_state.borrow_mut().change_current_chart(points_arg);
            }
            ChartExplorerCommands::OpenFile => {
                let file_path = if let Some(path_arg) = args.get(1) {
                    Path::new(path_arg)
                } else {
                    return Err(CommandError::NotEnoughArguments.into());
                };
                if !file_path.exists() {
                    let file_name = file_path.to_str().unwrap_or_default();
                    return Err(CommandError::InvalidArguments(String::from(file_name)).into());
                }
                self.add_chart_from_file(file_path.to_path_buf())?;
            }
        };

        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, rect: Rect) {
        let app_state = self.app_state.borrow();
        let (title_color, status_color, block_color) =
            if app_state.mode() == ApplicationMode::Explorer {
                (Color::Yellow, Color::Gray, Color::Yellow)
            } else {
                (Color::LightYellow, Color::DarkGray, Color::LightYellow)
            };
        let items: Vec<ListItem> = app_state
            .charts()
            .iter()
            .map(|chart_rc| {
                let chart = chart_rc.borrow();
                let title_line = Line::from(Span::styled(
                    chart.metadata.title.clone(),
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ));
                let desc_line = Line::from(Span::styled(
                    chart.metadata.description.clone(),
                    Style::default().fg(status_color),
                ));
                ListItem::new(vec![title_line, desc_line])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(CHART_EXPLORER_WIDGET_NAME)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(block_color)),
            )
            .highlight_symbol("➤ ");
        self.list_state.select(Some(app_state.current_chart_id()));
        f.render_stateful_widget(list, rect, &mut self.list_state);
    }

    fn add_chart_from_file(&self, path: PathBuf) -> Result<()> {
        let extension = if let Some(extension) = path.extension() {
            extension.to_str().ok_or(FileError::ExtensionParseError)?
        } else {
            return Err(anyhow!(FileError::PathParseError));
        };
        let parser = self
            .file_parsers
            .get(&FileType::from_str(&extension)?)
            .ok_or(FileError::UnsupportedType)?;
        let data = parser.parse_signal_file(path.to_str().ok_or(FileError::PathParseError)?, 0)?;
        self.app_state.borrow_mut().add_chart(data);

        Ok(())
    }
}
