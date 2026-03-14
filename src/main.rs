use std::io::Stdout;

use crate::{
    app::App,
    config::{VisualizerStyle, config},
    filter::normal::NormalFilter,
    visualizer::{
        Direction, Orientation, mono::MonoVisualizer, stereo::StereoVisualizer,
        waterfall::Waterfall,
    },
};
use clap::Parser;
use ratatui::{Terminal, prelude::CrosstermBackend};

mod app;
mod capture;
mod config;
mod event;
mod fft;
mod filter;
mod spectrum;
mod ui;
mod visualizer;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = factory(terminal).await;
    // match (args.stereo, args.centered, args.inverted, args.waterfall) {
    //     (false, false, false, true) => App::new(Waterfall::default()).run(terminal).await,
    //     (true, true, _, false) => {
    //         App::new(StereoVisualizer::default().centered())
    //             .run(terminal)
    //             .await
    //     }
    //     (true, _, true, false) => {
    //         App::new(StereoVisualizer::default().inverted())
    //             .run(terminal)
    //             .await
    //     }
    //     (true, _, _, false) => App::new(StereoVisualizer::default()).run(terminal).await,
    //     (_, _, _, _) => App::new(MonoVisualizer::default()).run(terminal).await,
    // };
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
    normal: bool,
    #[arg(short, long)]
    inverted: bool,
    #[arg(short, long)]
    waterfall: bool,
    #[arg(short, long)]
    mono: bool,
    #[arg(short, long)]
    vertical: bool,
    #[arg(short = 'H', long)]
    horizontal: bool,
}

// i hate this function
async fn factory(terminal: Terminal<CrosstermBackend<Stdout>>) -> color_eyre::Result<()> {
    let config = config();
    let args = Args::parse();

    if [args.stereo, args.waterfall, args.mono]
        .into_iter()
        .filter(|&p| p)
        .count()
        > 1
    {
        panic!("pick either stereo, waterfall, mono, or none of them")
    }

    if [args.centered, args.normal, args.inverted]
        .into_iter()
        .filter(|&p| p)
        .count()
        > 1
    {
        panic!("pick either centered, normal, inverted or none of them")
    }

    if args.vertical && args.horizontal {
        panic!("pick either vertical or horizontal")
    }

    let direction = if args.horizontal {
        Direction::Horizontal
    } else {
        config.visualizer.direction.into()
    };

    let orientation = if args.centered {
        Orientation::Centered
    } else if args.inverted {
        Orientation::Inverted
    } else {
        config.visualizer.orientation.into()
    };

    let bars = config.visualizer.bars;
    let scale = config.visualizer.scale;

    // let rate = config.input.rate;
    // let quant = config.input.quant;

    if args.stereo || config.visualizer.style == VisualizerStyle::Stereo {
        let mut stereo = StereoVisualizer::new(bars, direction, orientation);
        if args.normal {
            stereo = stereo.filter(Box::new(NormalFilter::default()));
        }
        return App::new(stereo).run(terminal).await;
    }

    if args.waterfall || config.visualizer.style == VisualizerStyle::Waterfall {
        let curves = config.visualizer.curves;
        let waterfall = Waterfall::new(curves, bars, scale);
        return App::new(waterfall).run(terminal).await;
    }

    let mut mono = MonoVisualizer::new(bars, scale, direction, orientation);
    mono.init();
    if args.normal {
        mono = mono.filter(Box::new(NormalFilter::default()));
    }

    App::new(mono).run(terminal).await
}

impl From<config::Direction> for Direction {
    fn from(value: config::Direction) -> Self {
        match value {
            config::Direction::Vertical => Direction::Vertical,
            config::Direction::Horizontal => Direction::Horizontal,
        }
    }
}

impl From<config::Orientation> for Orientation {
    fn from(value: config::Orientation) -> Self {
        match value {
            config::Orientation::Normal => Orientation::Normal,
            config::Orientation::Centered => Orientation::Centered,
            config::Orientation::Inverted => Orientation::Inverted,
        }
    }
}
