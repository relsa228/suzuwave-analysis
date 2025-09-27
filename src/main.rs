use suzuwave::{app::App, models::graphic_view::point::Point};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let data: Vec<Point> = (0..100)
        .map(|x| Point::new(x as f64, (x as f64 / 5.0).sin()))
        .collect();

    let result = App::new(data).run(terminal);
    ratatui::restore();
    result
}
