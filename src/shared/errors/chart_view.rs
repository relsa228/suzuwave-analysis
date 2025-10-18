use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChartViewError {
    #[error("No current chart found")]
    NoCurrentChart,
}
