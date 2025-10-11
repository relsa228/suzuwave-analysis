use crate::models::graphic_view::point::Point;
use rustfft::{FftPlanner, num_complex::Complex};

pub struct GraphicProcessService {
    planner: FftPlanner<f64>,
}

impl GraphicProcessService {
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
        }
    }

    pub fn fft_forward(&mut self, points: Vec<Point>) -> Vec<Point> {
        let mut buffer: Vec<Complex<f64>> = points
            .iter()
            .map(|point| Complex::new(point.y(), 0.0))
            .collect();

        let fft = self.planner.plan_fft_forward(buffer.len());
        fft.process(&mut buffer);

        buffer
            .iter()
            .enumerate()
            .map(|(i, c)| Point::new(i as f64, c.norm()))
            .collect()
    }
}
