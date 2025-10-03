// TODO: add --help && --version
// add about && help for views (similar content with --help and --version)
// add fft, filters, wavelet
// add files name tabs
// basic impl for file explorer
// add docs
// add mouse events

use clap::Parser;
use std::path::Path;
use suzuwave::app::App;

#[derive(Parser, Debug)]
#[command(author, disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(short)]
    f: Option<String>,

    #[arg(short)]
    v: bool,
    #[arg(long)]
    version: bool,

    #[arg(short)]
    h: bool,
    #[arg(long)]
    help: bool,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = if let Some(f) = args.f {
        let path = Path::new(f.as_str());
        App::new(if path.exists() { Some(path) } else { None }).run(terminal)
    } else {
        App::new(None).run(terminal)
    };
    ratatui::restore();
    app
}
