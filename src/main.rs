use crate::{
    app::App,
    visualize::{mono::MonoVisualizer, stereo::StereoVisualizer},
};
use clap::Parser;

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
    let args = Args::parse();
    let is_stereo = args.stereo;
    let result = if is_stereo {
        App::<StereoVisualizer>::new().run(terminal).await
    } else {
        App::<MonoVisualizer>::new().run(terminal).await
    };
    ratatui::restore();
    result
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    stereo: bool,
}
