use crate::models::graphic_view::{plot::GraphicViewPlot, point::Point};
use kofft::{Complex32, stft::stft, wavelet::haar_forward_inplace_stack, window::hann};
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

    pub fn fft_forward(&mut self, plot: &GraphicViewPlot) -> Vec<Point> {
        let mut buffer: Vec<Complex<f64>> =
            plot.data.iter().map(|p| Complex::new(p.y(), 0.0)).collect();

        let fft = self.planner.plan_fft_forward(buffer.len());
        fft.process(&mut buffer);

        let n = buffer.len();
        let freq_res = plot.sample_rate / n as f32;

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
                let freq = (i as f64 - half as f64) * freq_res as f64;
                let norm = c.norm() / n as f64 * 2.0;
                if freq < 0.0 || norm < 0.1 {
                    None
                } else {
                    Some(Point::new(freq, norm))
                }
            })
            .collect()
    }

    pub fn stft_forward(
        &self,
        plot: &GraphicViewPlot,
        window_size: usize,
        hop_size: usize,
    ) -> Vec<Point> {
        let y: Vec<f32> = plot.data.iter().map(|point| point.y() as f32).collect();
        let window = hann(window_size);
        let hop_size = hop_size;
        let fft = vec![Complex32::new(0.0, 0.0); window.len()];
        let mut frames = vec![fft; (y.len() + hop_size - 1) / hop_size];

        if let Err(err) = stft(&y, &window, hop_size, &mut frames[..]) {
            eprintln!("STFT error: {:?}", err);
            return vec![Point::new(0.0, 0.0)];
        }

        let res = frames
            .iter()
            .enumerate()
            .map(|(t, frame)| {
                let avg = frame
                    .iter()
                    .map(|c| c.re.powi(2) + c.im.powi(2))
                    .sum::<f32>() as f64
                    / frame.len() as f64;
                Point::new(t as f64, avg)
            })
            .collect();
        res
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

    pub fn haar_wavelet_transform(&self, plot: &GraphicViewPlot) -> Vec<Point> {
        let y: Vec<f32> = plot.data.iter().map(|point| point.y() as f32).collect();

        let mut freq_data: [f32; u32::MAX as usize] = [0.0; u32::MAX as usize];
        let len_to_copy = y.len().min(u32::MAX as usize);
        freq_data[..len_to_copy].copy_from_slice(&y[..len_to_copy]);

        let freq_data_ref: &[f32; u32::MAX as usize] = &freq_data;
        let mut approx = [0.0; (u32::MAX / 2) as usize];
        let mut detail = [0.0; (u32::MAX / 2) as usize];

        haar_forward_inplace_stack(freq_data_ref, &mut approx[..], &mut detail[..]);

        let approx_end_index = y.len().div_ceil(2);
        let approx = approx
            .to_vec()
            .drain(0..approx_end_index)
            .collect::<Vec<f32>>();
        approx
            .iter()
            .enumerate()
            .map(|(i, v)| Point::new(i as f64, *v as f64))
            .collect::<Vec<Point>>()
    }
}
