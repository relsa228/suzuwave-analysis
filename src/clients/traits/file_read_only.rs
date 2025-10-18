use anyhow::Result;

use crate::models::chart_view::chart::chart_model::ChartModel;

pub trait FileReadOnly {
    /// Parse data file
    ///
    /// Collects signal data from a file and returns a vector of points.
    ///
    /// * `path`: The path to the file to be parsed.
    /// * `channel`: The channel to be parsed.
    fn parse_signal_file(&self, path: &str, channel: usize) -> Result<ChartModel>;
}
