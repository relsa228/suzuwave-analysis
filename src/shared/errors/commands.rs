use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("No such command: {0}, use :h for help")]
    CommandSyntax(String),

    #[error("Invalid argument: {0}, use :h for help")]
    InvalidArguments(String),

    #[error("Not enough arguments, use :h for help")]
    NotEnoughArguments,

    #[error("Try some commands, use :h for help")]
    EmptyCommand,

    #[error("No chart selected, use :of to open a chart file")]
    NoChart,
}
