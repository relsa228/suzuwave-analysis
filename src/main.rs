use crate::app::App;

pub mod app;
pub mod components;
pub mod models;
pub mod states;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
