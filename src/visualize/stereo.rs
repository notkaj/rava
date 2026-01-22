use ratatui::style::Color;

use crate::{
    filter::Filter,
    spectrum::{spectral::Spectral, stereo::StereoSpectrum},
    visualize::visual::{COLORS, DEFAULT_COLOR_INDEX, Mode, Visual},
};

pub struct StereoVisualizer<T: Filter> {
    pub color_index: usize,
    spectrum: StereoSpectrum,
    left_out: Vec<u32>,
    right_out: Vec<u32>,
    filter: T,
    pub mode: Mode,
}

impl<T: Filter + Default> Default for StereoVisualizer<T> {
    fn default() -> Self {
        Self::new(DEFAULT_COLOR_INDEX, Default::default(), Default::default())
    }
}

impl<T: Filter> StereoVisualizer<T> {
    pub fn new(color_index: usize, filter: T, spectrum: StereoSpectrum) -> Self {
        let bars = spectrum.ranges;
        let left_out = vec![0; bars];
        let right_out = vec![0; bars];
        let mode = Default::default();
        Self {
            color_index,
            spectrum,
            left_out,
            right_out,
            filter,
            mode,
        }
    }
}

impl<T: Filter> Visual for StereoVisualizer<T> {
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
        self.spectrum.max().unwrap_or_default()
    }
}
