use crate::app::App;

pub mod app;
pub mod capture;
pub mod event;
pub mod fft;
pub mod spectrum;
pub mod ui;
pub mod visualizer;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
