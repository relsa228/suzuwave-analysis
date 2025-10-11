use anyhow::Result;
use ratatui::widgets::GraphType;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    clients::traits::file_read_only::FileReadOnly,
    models::{
        files::{signal_file::SignalFile, signal_header::SignalHeader},
        graphic_view::{plot::GraphicViewPlot, point::Point},
    },
    shared::errors::files::FileError,
};

type ParsedFileData = (Vec<Point>, f32);

const VIBRIC_SIGNATURE: &[u8] = b"TMB1";

pub struct VibricReadingClient {}

impl VibricReadingClient {
    pub fn new() -> Self {
        VibricReadingClient {}
    }

    fn read_u32<R: Read>(&self, reader: &mut R) -> Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_f32<R: Read>(&self, reader: &mut R) -> Result<f32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn parse_bin_file(&self, path: &str, channel: usize) -> Result<ParsedFileData> {
        let mut reader = BufReader::new(File::open(path)?);
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature)?;
        if &signature != VIBRIC_SIGNATURE {
            return Err(FileError::VibricSignature.into());
        }

        let header = SignalHeader {
            signature,
            channels: self.read_u32(&mut reader)?,
            sample_size: self.read_u32(&mut reader)?,
            spectral_lines: self.read_u32(&mut reader)?,
            cutoff_freq: self.read_u32(&mut reader)?,
            freq_resolution: self.read_f32(&mut reader)?,
            block_time: self.read_f32(&mut reader)?,
            total_time: self.read_u32(&mut reader)?,
            blocks_set: self.read_u32(&mut reader)?,
            data_size: self.read_u32(&mut reader)?,
            blocks_received: self.read_u32(&mut reader)?,
            max_value: self.read_f32(&mut reader)?,
            min_value: self.read_f32(&mut reader)?,
        };

        let sample_rate = header.freq_resolution * header.sample_size as f32;

        let mut data = Vec::with_capacity(header.data_size as usize);
        for _ in 0..header.data_size {
            data.push(self.read_f32(&mut reader)?);
        }

        let signal = SignalFile::new(header, data);
        let header = &signal.header;
        let dt = header.block_time / header.sample_size as f32;
        let mut points = Vec::new();

        for (i, &y) in signal.data.iter().enumerate() {
            if i % header.channels as usize == channel {
                let sample_index = i / header.channels as usize;
                let x = sample_index as f32 * dt;
                points.push(Point::new(x as f64, y as f64));
            }
        }

        Ok((points, sample_rate))
    }
}

impl FileReadOnly for VibricReadingClient {
    fn parse_signal_file(&self, path: &str, channel: usize) -> Result<GraphicViewPlot> {
        let (points, sample_rate) = self.parse_bin_file(path, channel)?;
        Ok(GraphicViewPlot::new(points, GraphType::Line, sample_rate))
    }
}
