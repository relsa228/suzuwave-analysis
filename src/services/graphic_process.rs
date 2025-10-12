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

    pub fn fft_forward(&mut self, points: Vec<Point>, sample_rate: f64) -> Vec<Point> {
        let mut buffer: Vec<Complex<f64>> =
            points.iter().map(|p| Complex::new(p.y(), 0.0)).collect();

        let fft = self.planner.plan_fft_forward(buffer.len());
        fft.process(&mut buffer);

        let n = buffer.len();
        let freq_res = sample_rate / n as f64;

        let mut shifted = vec![Complex::new(0.0, 0.0); n];
        let half = n / 2;
        for i in 0..half {
            shifted[i] = buffer[i + half];
            shifted[i + half] = buffer[i];
        }

        shifted
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let freq = (i as f64 - half as f64) * freq_res;
                Point::new(freq, c.norm() / n as f64 * 2.0)
            })
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
                    FftFilterType::BandStop(low, high) => freq as f64 <= low || freq as f64 >= high,
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
