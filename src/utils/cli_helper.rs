use std::path::{Path, PathBuf};

use crate::models::cli::args::Args;

pub struct CliHelper {
    args: Args,
}

impl CliHelper {
    pub fn new(args: Args) -> Self {
        CliHelper { args }
    }

    pub fn process_path(&self) -> Option<PathBuf> {
        if let Some(f) = self.args.f.clone() {
            let path = Path::new(f.as_str());
            if path.exists() {
                Some(path.to_path_buf())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn help(&self) -> bool {
        if self.args.help || self.args.h {
            println!("CLI usage: suzu [OPTIONS]");
            println!("Options:");
            println!("  <NONE>          Default open option");
            println!("  -f <FILE>       Specify the input signal file");
            println!("  -h, --help      Display this help message");
            println!("  -v, --version   Display the version");
            true
        } else {
            false
        }
    }

    pub fn version(&self) -> bool {
        if self.args.version || self.args.v {
            println!("suzu v{}", env!("CARGO_PKG_VERSION"));
            true
        } else {
            false
        }
    }
}
