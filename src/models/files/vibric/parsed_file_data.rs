use crate::models::chart_view::chart::point::Point;

pub struct ParsedFileData {
    pub data: Vec<Point>,
    pub sample_rate: f32,
    pub chart_title: String,
}

impl ParsedFileData {
    pub fn new(data: Vec<Point>, sample_rate: f32, chart_title: String) -> Self {
        Self {
            data,
            sample_rate,
            chart_title,
        }
    }
}
