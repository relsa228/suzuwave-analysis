use strum_macros::Display;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr, PartialEq, Hash, Eq, Clone, Copy, Display)]
pub enum ChartTransform {
    #[strum(serialize = "Standard")]
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

impl Default for ChartTransform {
    fn default() -> Self {
        ChartTransform::Standard
    }
}
