use anyhow::Result;
use ratatui::widgets::GraphType;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::{
    clients::traits::file_read_only::FileReadOnly,
    models::{
        chart_view::chart::{chart_model::ChartModel, point::Point},
        files::{
            signal_file::SignalFile, signal_header::SignalHeader,
            vibric::parsed_file_data::ParsedFileData,
        },
    },
    shared::{constants::vibric::VIBRIC_SIGNATURE, errors::files::FileError},
};

pub struct VibricReadingClient;

impl VibricReadingClient {
    pub fn new() -> Self {
        VibricReadingClient
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

    /// Vibric file parsing
    ///
    /// Parsing a Vibric file data according to the Vibric file format specification,
    /// converting it to a ParsedFileData struct
    ///
    /// ---
    ///
    /// * `path`: file path to the Vibric file
    /// * `channel`: channel number to parse
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

        let path_file = Path::new(path);

        let parsed_data = ParsedFileData::new(
            points,
            sample_rate,
            String::from(
                path_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ),
        );
        Ok(parsed_data)
    }
}

impl FileReadOnly for VibricReadingClient {
    fn parse_signal_file(&self, path: &str, channel: usize) -> Result<ChartModel> {
        let parsed_data = self.parse_bin_file(path, channel)?;
        Ok(ChartModel::new(
            parsed_data.data,
            GraphType::Line,
            parsed_data.sample_rate,
            &parsed_data.chart_title,
            None,
        ))
    }
}
