// TODO:
// add stft
//
// basic impl for chart explorer
//
// add docs && code refactoring
//
// add mouse events

use clap::Parser;
use suzu::{app::App, models::cli::args::Args, utils::cli_helper::CliHelper};

fn main() -> color_eyre::Result<()> {
    let cli_helper = CliHelper::new(Args::parse());
    if cli_helper.version() || cli_helper.help() {
        return Ok(());
    }

    color_eyre::install()?;
    ratatui::restore();
    let terminal = ratatui::init();
    App::new(cli_helper.process_path()).run(terminal)
}
