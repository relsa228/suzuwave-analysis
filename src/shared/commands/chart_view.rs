use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Eq)]
pub enum ChartViewCommands {
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
    #[strum(serialize = ":sft")]
    ShortTimeFourierTransform,
    #[strum(serialize = ":flp")]
    FftFilterLowPass,
    #[strum(serialize = ":fhp")]
    FftFilterHighPass,
    #[strum(serialize = ":fbp")]
    FftFilterBandPass,
    #[strum(serialize = ":fbs")]
    FftFilterBandStop,
    #[strum(serialize = ":hwt")]
    HaarWaveletTransform,
}
