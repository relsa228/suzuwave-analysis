use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphicViewError {
    #[error("No current plot found")]
    NoCurrentPlot,
}
