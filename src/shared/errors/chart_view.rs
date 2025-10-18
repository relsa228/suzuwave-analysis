use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChartViewError {
    #[error("No current plot found")]
    NoCurrentPlot,
}
