use crate::models::graphic_widget::point::Point;

pub struct GraphicWidgetState {
    datasets: Vec<Vec<Point>>,
    current_dataset: Vec<Point>,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl GraphicWidgetState {
    pub fn new() -> Self {
        let data: Vec<Point> = (0..100)
            .map(|x| Point::new(x as f64, (x as f64 / 5.0).sin()))
            .collect();
        Self {
            datasets: vec![data.clone()],
            current_dataset: data,
            x_min: 0.0,
            x_max: 20.0,
            y_min: -1.2,
            y_max: 1.2,
        }
    }

    pub fn current_dataset(&self) -> Vec<(f64, f64)> {
        self.current_dataset
            .iter()
            .map(|point| (point.x(), point.y()))
            .collect()
    }

    pub fn x_min(&self) -> f64 {
        self.x_min
    }

    pub fn x_max(&self) -> f64 {
        self.x_max
    }

    pub fn y_min(&self) -> f64 {
        self.y_min
    }

    pub fn y_max(&self) -> f64 {
        self.y_max
    }

    pub fn x_max_add(&mut self, value: f64) {
        self.x_max += value;
    }

    pub fn y_max_add(&mut self, value: f64) {
        self.y_max += value;
    }

    pub fn x_min_add(&mut self, value: f64) {
        self.x_min += value;
    }

    pub fn y_min_add(&mut self, value: f64) {
        self.y_min += value;
    }

    pub fn set_x_max(&mut self, value: f64) {
        self.x_max = value;
    }

    pub fn set_y_max(&mut self, value: f64) {
        self.y_max = value;
    }

    pub fn set_x_min(&mut self, value: f64) {
        self.x_min = value;
    }

    pub fn set_y_min(&mut self, value: f64) {
        self.y_min = value;
    }
}
