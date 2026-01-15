use ratatui::style::Color;

use crate::filter::{ExperimentalFilter, Filter};
use crate::spectrum::average::AverageSpectrum;
use crate::spectrum::spectral::Spectral;

const DEFAULT_BAR_COUNT: usize = 72;
const DEFAULT_COLOR_INDEX: usize = 5;
pub const COLORS: [Color; 8] = [
    Color::White,
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Gray,
];

pub struct Visualizer<T: Filter> {
    pub color_index: usize,
    spectrum: AverageSpectrum,
    out: Vec<u32>,
    filter: T,
    pub mode: Mode,
}

impl<T: Filter + Default> Default for Visualizer<T> {
    fn default() -> Self {
        Self::new(DEFAULT_BAR_COUNT, DEFAULT_COLOR_INDEX, Default::default())
    }
}

impl Default for Visualizer<ExperimentalFilter> {
    fn default() -> Self {
        let filter = ExperimentalFilter::new_default(DEFAULT_BAR_COUNT);
        Self::new(DEFAULT_BAR_COUNT, DEFAULT_COLOR_INDEX, filter)
    }
}

impl<T: Filter> Visualizer<T> {
    pub fn new(bars: usize, color_index: usize, filter: T) -> Self {
        let spectrum = AverageSpectrum::new(bars);
        let out = vec![0; bars];
        Self {
            color_index,
            spectrum,
            out,
            filter,
            mode: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.spectrum.init();
    }

    pub fn update(&mut self) {
        self.spectrum.update();
        self.filter.apply(&self.spectrum.amps, &mut self.out);
    }

    pub fn output(&self) -> &[u32] {
        &self.out
    }

    pub fn add_bar(&mut self) {
        self.spectrum.add_range();
        self.out.push(0);
    }

    pub fn remove_bar(&mut self) {
        self.spectrum.remove_range();
        self.out.pop();
    }

    pub fn increment_scale(&mut self) {
        self.spectrum.adjust_scale(1.0);
    }

    pub fn decrement_scale(&mut self) {
        self.spectrum.adjust_scale(-1.0);
    }

    pub fn color(&self) -> Color {
        COLORS[self.color_index]
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % COLORS.len()
    }

    pub fn prev_color(&mut self) {
        if self.color_index > 0 {
            self.color_index -= 1;
        } else {
            self.color_index = COLORS.len() - 1;
        }
    }

    pub fn sample_rate(&self) -> usize {
        self.spectrum.sample_rate()
    }

    pub fn channels(&self) -> usize {
        self.spectrum.channels()
    }

    pub fn sample_len(&self) -> usize {
        self.spectrum.sample_len
    }

    pub fn bars(&self) -> usize {
        self.spectrum.ranges
    }

    pub fn input_max(&self) -> u32 {
        self.spectrum.max().unwrap_or_default()
    }
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Default,
    ColorPick,
    ShowStats,
    ShowKeys,
    ShowInput,
}
