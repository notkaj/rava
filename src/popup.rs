use ratatui::style::Color;

pub const VERT_PERCENT: u16 = 30;
pub const HORIZ_PERCENT: u16 = 20;

pub struct StatsPopup {
    pub color: Color,
    pub sample_rate: usize,
    pub sample_len: usize,
    pub channels: usize,
}

impl StatsPopup {
    pub fn new(color: Color, sample_rate: usize, sample_len: usize, channels: usize) -> Self {
        Self {
            color,
            sample_rate,
            sample_len,
            channels,
        }
    }
}

pub struct KeysPopup {
    pub color: Color,
}

impl KeysPopup {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

pub struct ColorPickPopup {
    pub colors: &'static [Color],
    pub index: usize,
    pub color: Color,
}

impl ColorPickPopup {
    pub fn new(color: Color, colors: &'static [Color], index: usize) -> Self {
        Self {
            colors,
            index,
            color,
        }
    }
}

pub struct InputPopup {
    pub color: Color,
    pub max: u32,
    pub channels: usize,
    pub rate: usize,
}

impl InputPopup {
    pub fn new(color: Color, max: u32, channels: usize, rate: usize) -> Self {
        Self {
            color,
            max,
            channels,
            rate,
        }
    }
}
