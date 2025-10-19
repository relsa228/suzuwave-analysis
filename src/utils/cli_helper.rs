use std::path::{Path, PathBuf};

use crate::models::cli::args::Args;

pub struct CliHelper {
    args: Args,
}

impl CliHelper {
    pub fn new(args: Args) -> Self {
        CliHelper { args }
    }

    /// Process the input file path
    ///
    /// Returns the path from the `f` argument if it exists, otherwise None
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

    /// Display help message
    ///
    /// Prints the help message to the console
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

    /// Display version message
    ///
    /// Prints the version message to the console
    pub fn version(&self) -> bool {
        if self.args.version || self.args.v {
            println!("suzu v{}", env!("CARGO_PKG_VERSION"));
            true
        } else {
            false
        }
    }
}
