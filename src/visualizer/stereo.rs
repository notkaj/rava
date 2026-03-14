use better_default::Default;
use ratatui::style::Color;

use super::{COLORS, DEFAULT_COLOR_INDEX, Direction, Mode, Orientation, Visualizer};
use crate::{
    filter::{Filter, cava::CavaFilter},
    spectrum::{Spectrum, stereo::StereoSpectrum},
};

#[derive(Default)]
pub struct StereoVisualizer {
    #[default(DEFAULT_COLOR_INDEX)]
    pub color_index: usize,
    spectrum: StereoSpectrum,
    pub left_out: Vec<u32>,
    pub right_out: Vec<u32>,
    #[default(Box::new(CavaFilter::default()))]
    filter: Box<dyn Filter>,
    pub mode: Mode,
    pub direction: Direction,
    pub orientation: Orientation,
}

impl StereoVisualizer {
    pub fn new(bars: usize, direction: Direction, orientation: Orientation) -> Self {
        let spectrum = StereoSpectrum::new(bars);
        let filter = Box::new(CavaFilter::default());
        let left_out = vec![0; bars];
        let right_out = vec![0; bars];
        let color_index = DEFAULT_COLOR_INDEX;
        let mode = Default::default();

        Self {
            color_index,
            spectrum,
            left_out,
            right_out,
            filter,
            mode,
            direction,
            orientation,
        }
    }

    pub fn init(&mut self) {
        self.spectrum.init();
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn centered(mut self) -> Self {
        self.orientation = Orientation::Centered;
        self
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn inverted(mut self) -> Self {
        self.orientation = Orientation::Inverted;
        self
    }

    #[must_use = "builder pattern"]
    pub fn filter(mut self, filter: Box<dyn Filter>) -> Self {
        self.filter = filter;
        self
    }
}

impl Visualizer for StereoVisualizer {
    fn update(&mut self) {
        self.spectrum.update();
        self.filter
            .apply(&self.spectrum.left_amps, &mut self.left_out);
        self.filter
            .apply(&self.spectrum.right_amps, &mut self.right_out);
    }

    fn add_bar(&mut self) {
        self.spectrum.add_range();
        self.left_out.push(0);
        self.right_out.push(0);
    }

    fn remove_bar(&mut self) {
        self.spectrum.remove_range();
        self.left_out.pop();
        self.right_out.pop();
    }

    fn increment_scale(&mut self) {
        self.spectrum.adjust_scale(1.0);
    }

    fn decrement_scale(&mut self) {
        self.spectrum.adjust_scale(-1.0);
    }

    fn color(&self) -> Color {
        COLORS[self.color_index]
    }

    fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % COLORS.len()
    }

    fn prev_color(&mut self) {
        if self.color_index > 0 {
            self.color_index -= 1;
        } else {
            self.color_index = COLORS.len() - 1;
        }
    }

    fn sample_rate(&self) -> usize {
        self.spectrum.sample_rate()
    }

    fn channels(&self) -> usize {
        self.spectrum.channels()
    }

    fn sample_len(&self) -> usize {
        self.spectrum.sample_len
    }

    fn bars(&self) -> usize {
        self.spectrum.ranges
    }

    fn input_max(&self) -> u32 {
        self.spectrum.max_amp().unwrap_or_default()
    }

    fn color_index(&self) -> usize {
        self.color_index
    }

    fn get_mode(&self) -> Mode {
        self.mode
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
