use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChartProcessingError {
    #[error("STFT error. Try changing the window size or hop size.")]
    StftError,

    #[error("To perform this operation the chart should be in the STANDARD view")]
    NonStandard,
}
