use crate::{
    clients::{files::vibric::VibricReadingClient, traits::file_read_only::FileReadOnly},
    models::files::file_types::FileType,
    shared::{
        commands::chart_explorer::ChartExplorerCommands,
        constants::command::DEFAULT_COMMAND_PREFIX,
        errors::{commands::CommandError, files::FileError},
    },
    states::app::ApplicationState,
};
use anyhow::Result;
use ratatui::{Frame, layout::Rect};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

pub struct ChartExplorerComponent {
    app_state: Rc<RefCell<ApplicationState>>,
    file_parsers: HashMap<FileType, Box<dyn FileReadOnly>>,
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
        };
        if let Some(file) = initial_signal_file {
            let _ = instance.add_chart_from_file(file);
        }
        instance
    }

    pub fn update_from_state(&mut self) -> Result<()> {
        let state_borrow = self.app_state.borrow();
        if let Some(cmd) = state_borrow.command() {
            drop(state_borrow);
            let args = cmd.split_whitespace().collect::<Vec<&str>>();
            if args.is_empty() || args[0] == DEFAULT_COMMAND_PREFIX {
                return Err(CommandError::EmptyCommand.into());
            }
            if let Ok(command) = ChartExplorerCommands::from_str(args[0]) {
                match command {
                    ChartExplorerCommands::OpenFile => {
                        if let Some(path_arg) = args.get(1) {
                            let file_path = Path::new(path_arg);
                            if !file_path.exists() {
                                return Err(CommandError::InvalidArguments(String::from(
                                    *path_arg,
                                ))
                                .into());
                            }
                            self.add_chart_from_file(file_path.to_path_buf())?;
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                    ChartExplorerCommands::CloseWorkingView => {
                        self.app_state.borrow_mut().delete_chart();
                    }
                    ChartExplorerCommands::SwitchWorkingView => {
                        if let Some(points_arg) = args.get(1) {
                            self.app_state.borrow_mut().change_current_chart(
                                points_arg.parse::<u32>().map_err(|_| {
                                    CommandError::InvalidArguments(String::from(*points_arg))
                                })?,
                            );
                        } else {
                            return Err(CommandError::NotEnoughArguments.into());
                        }
                    }
                };
                self.app_state.borrow_mut().set_command(None);
            }
        }
        Ok(())
    }

    pub fn render(&mut self, _f: &mut Frame, _rect: Rect) {}

    fn add_chart_from_file(&self, path: PathBuf) -> Result<()> {
        if let Some(extension) = path.extension() {
            let parser = self
                .file_parsers
                .get(&FileType::from_str(
                    &extension.to_str().ok_or(FileError::ExtensionParseError)?,
                )?)
                .ok_or(FileError::UnsupportedType)?;
            let data =
                parser.parse_signal_file(path.to_str().ok_or(FileError::PathParseError)?, 0)?;
            self.app_state.borrow_mut().add_chart(data);
        }
        Ok(())
    }
}
