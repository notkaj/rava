use crate::{
    app::App,
    visualize::{mono::MonoVisualizer, stereo::StereoVisualizer, waterfall::Waterfall},
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
    // let result = if is_stereo {
    //     App::<StereoVisualizer>::default().run(terminal).await
    // } else {
    //     App::<MonoVisualizer>::default().run(terminal).await
    // };
    let result = match (args.stereo, args.centered, args.inverted, args.waterfall) {
        (false, false, false, true) => App::new(Waterfall::default()).run(terminal).await,
        (true, true, _, false) => {
            App::new(StereoVisualizer::default().centered())
                .run(terminal)
                .await
        }
        (true, _, true, false) => {
            App::new(StereoVisualizer::default().inverted())
                .run(terminal)
                .await
        }
        (true, _, _, false) => App::new(StereoVisualizer::default()).run(terminal).await,
        (_, _, _, _) => App::new(MonoVisualizer::default()).run(terminal).await,
    };
    ratatui::restore();
    result
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    stereo: bool,
    #[arg(short, long)]
    centered: bool,
    #[arg(short, long)]
    inverted: bool,
    #[arg(short, long)]
    waterfall: bool,
    // #[arg(short, long)]
    // vertical: bool,
}
