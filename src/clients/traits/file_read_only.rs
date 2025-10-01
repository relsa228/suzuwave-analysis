use anyhow::Result;

use crate::models::graphic_view::point::Point;

pub trait FileReadOnly {
    /// Parse data file
    ///
    /// Collects signal data from a file and returns a vector of points.
    ///
    /// * `path`: The path to the file to be parsed.
    /// * `channel`: The channel to be parsed.
    fn parse_signal_file(&self, path: &str, channel: usize) -> Result<Vec<Point>>;
}
