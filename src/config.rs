use better_default::Default;
use serde::Deserialize;
use std::{error::Error, fs};

const DEFAULT_CONFIG_PATHS: [&str; 1] = ["~/.config/rava/config.toml"];

pub(crate) fn config() -> Config {
    for path in DEFAULT_CONFIG_PATHS {
        match fs::exists(path) {
            Ok(true) => {
                let res = parse(path);
                match res {
                    Ok(c) => return c,
                    Err(e) => panic!("Issue with config file: {}", e),
                }
            }
            Ok(false) => continue,
            Err(_) => panic!("Issue with finding config file"),
        }
    }
    Default::default()
}

fn parse(path: &str) -> Result<Config, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let config = toml::from_str(&text)?;
    Ok(config)
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Config {
    pub visualizer: Visualizer,
    // pub input: Input,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Visualizer {
    pub style: VisualizerStyle,
    pub direction: Direction,
    pub orientation: Orientation,
    #[default(36)]
    pub bars: usize,
    #[default(24.0)]
    pub scale: f32,
    // #[default(String::from("White"))]
    // pub color: String,
    #[default(30)]
    pub curves: usize,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub(crate) enum VisualizerStyle {
    #[default]
    Mono,
    Stereo,
    Waterfall,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) enum Orientation {
    #[default]
    Normal,
    Centered,
    Inverted,
}

#[allow(dead_code)]
#[derive(Default, Debug, Deserialize)]
pub(crate) struct Input {
    #[default(String::from("Pipewire"))]
    method: String,
    #[default(1024)]
    pub rate: u16,
    #[default(48000)]
    pub quant: u16,
}
