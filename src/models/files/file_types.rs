use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Hash, Eq)]
pub enum FileType {
    #[strum(serialize = "bin")]
    Vibric,
}
