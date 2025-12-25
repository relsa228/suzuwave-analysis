use strum_macros::Display;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Hash, Eq, Clone, Copy, Display)]
#[derive(Default)]
pub enum ChartTransform {
    #[strum(serialize = "Standard")]
    #[default]
    Standard,

    #[strum(serialize = "FFT")]
    Fft,

    #[strum(serialize = "STFT")]
    Stft,

    #[strum(serialize = "Haar")]
    WaveletHaar,

    #[strum(serialize = "Filtered")]
    Filtered,
}

