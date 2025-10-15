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
            .filter_map(|(i, c)| {
                let freq = (i as f64 - half as f64) * freq_res;
                let norm = c.norm() / n as f64 * 2.0;
                if freq < 0.0 || norm < 0.1 {
                    None
                } else {
                    Some(Point::new(freq, norm))
                }
            })
            .collect()
    }

    pub fn apply_fft_filter(&self, plot: &GraphicViewPlot, filter: FftFilterType) -> Vec<Point> {
        plot.data
            .iter()
            .filter_map(|p| {
                if match filter {
                    FftFilterType::LowPass(cutoff) => p.x() <= cutoff,
                    FftFilterType::HighPass(cutoff) => p.x() >= cutoff,
                    FftFilterType::BandPass(low, high) => p.x() >= low && p.x() <= high,
                    FftFilterType::BandStop(low, high) => p.x() <= low || p.x() >= high,
                } {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
