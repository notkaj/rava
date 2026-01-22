use crate::app::App;

mod app;
mod capture;
mod event;
mod fft;
mod filter;
mod spectrum;
mod ui;
mod visualize;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
