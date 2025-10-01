use crate::models::files::signal_header::SignalHeader;

pub struct SignalFile {
    pub header: SignalHeader,
    pub data: Vec<f32>,
}

impl SignalFile {
    pub fn new(header: SignalHeader, data: Vec<f32>) -> Self {
        SignalFile { header, data }
    }
}
