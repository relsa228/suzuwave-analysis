use crate::models::graphic_view::{plot::GraphicViewPlot, point::Point};
use rustfft::{FftPlanner, num_complex::Complex};

#[derive(Debug, Clone, Copy)]
pub enum FftFilterType {
    LowPass(f64),
    HighPass(f64),
    BandPass(f64, f64),
    BandStop(f64, f64),
}

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

        let n = buffer.len();
        buffer
            .iter()
            .enumerate()
            .map(|(i, c)| Point::new(i as f64, c.norm() / n as f64))
            .collect()
    }

    pub fn apply_filter(&self, plot: &GraphicViewPlot, filter: FftFilterType) -> Vec<Point> {
        let n = plot.data.len();
        let freq_resolution = plot.sample_rate / n as f32;

        plot.data
            .iter()
            .map(|p| {
                let freq = p.x() as f32 * freq_resolution;
                let pass = match filter {
                    FftFilterType::LowPass(cutoff) => freq as f64 <= cutoff,
                    FftFilterType::HighPass(cutoff) => freq as f64 >= cutoff,
                    FftFilterType::BandPass(low, high) => freq as f64 >= low && freq as f64 <= high,
                    FftFilterType::BandStop(low, high) => {
                        (freq as f64) < low || (freq as f64) > high
                    }
                };
                if pass {
                    Point::new(freq as f64, p.y())
                } else {
                    Point::new(freq as f64, 0.0)
                }
            })
            .collect()
    }
}
