use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Eq)]
pub enum GeneralCommands {
    #[strum(serialize = ":a")]
    About,
    #[strum(serialize = ":h")]
    Help,
    #[strum(serialize = ":ce")]
    OpenCloseChartsExplorer,

    #[strum(serialize = ":sf")]
    OpenSettings,

    #[strum(serialize = ":q")]
    Quit,
}
