pub const VERT_PERCENT: u16 = 30;
pub const HORIZ_PERCENT: u16 = 20;

pub struct StatsPopup {
    pub sample_rate: usize,
    pub sample_len: usize,
    pub channels: usize,
}

impl StatsPopup {
    pub fn new(sample_rate: usize, sample_len: usize, channels: usize) -> Self {
        Self {
            sample_rate,
            sample_len,
            channels,
        }
    }
}

#[derive(Default)]
pub struct KeysPopup {}

impl KeysPopup {
    pub fn new() -> Self {
        Self {}
    }
}
