use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Eq)]
pub enum GraphicViewCommands {
    #[strum(serialize = ":of")]
    OpenFile,

    #[strum(serialize = ":zi")]
    ZoomIn,
    #[strum(serialize = ":zo")]
    ZoomOut,
    #[strum(serialize = ":ml")]
    MoveLeft,
    #[strum(serialize = ":mr")]
    MoveRight,

    #[strum(serialize = ":fft")]
    FastFourierTransform,
    #[strum(serialize = ":fl")]
    FftFilterLowPass,

    #[strum(serialize = ":cwv")]
    CloseWorkingView,
    #[strum(serialize = ":swv")]
    SwitchWorkingView,
}
