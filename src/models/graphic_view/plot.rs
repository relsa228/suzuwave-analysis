use crate::models::graphic_view::point::Point;

const MARGIN: f64 = 0.07;

#[derive(Clone)]
pub struct GraphicViewPlot {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub data: Vec<Point>,
}

impl GraphicViewPlot {
    pub fn new(data: Vec<Point>) -> Self {
        let x_min = data
            .iter()
            .map(|p| p.x())
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(f64::MIN);
        let x_max = data
            .iter()
            .map(|p| p.x())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(f64::MAX);
        let y_min = data
            .iter()
            .map(|p| p.y())
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(f64::MIN + MARGIN)
            - MARGIN;
        let y_max = data
            .iter()
            .map(|p| p.y())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(f64::MAX - MARGIN)
            + MARGIN;

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            data,
        }
    }

    pub fn data_to_pure_coordinates(&self) -> Vec<(f64, f64)> {
        self.data
            .iter()
            .map(|point| (point.x(), point.y()))
            .collect::<Vec<(f64, f64)>>()
    }
}
