use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Eq)]
pub enum ChartExplorerCommands {
    #[strum(serialize = ":of")]
    OpenFile,
    #[strum(serialize = ":cwv")]
    CloseWorkingView,
    #[strum(serialize = ":swv")]
    SwitchWorkingView,
}
