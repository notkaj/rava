use ratatui::style::Color;

use crate::filter::{ExperimentalFilter, Filter};
use crate::spectrum::average::AverageSpectrum;
use crate::spectrum::spectral::Spectral;
use crate::visualize::visual::DEFAULT_COLOR_INDEX;
use crate::visualize::visual::Mode;
use crate::visualize::visual::{COLORS, Visual};

pub struct MonoVisualizer<T: Filter> {
    pub color_index: usize,
    spectrum: AverageSpectrum,
    out: Vec<u32>,
    filter: T,
    pub mode: Mode,
}

impl<T: Filter + Default> Default for MonoVisualizer<T> {
    fn default() -> Self {
        Self::new(DEFAULT_COLOR_INDEX, Default::default(), Default::default())
    }
}

impl Default for MonoVisualizer<ExperimentalFilter> {
    fn default() -> Self {
        let filter = ExperimentalFilter::new_default(72); // this will fuck up if the count is off
        let spectrum = Default::default();
        Self::new(DEFAULT_COLOR_INDEX, filter, spectrum)
    }
}

impl<T: Filter> MonoVisualizer<T> {
    pub fn new(color_index: usize, filter: T, spectrum: AverageSpectrum) -> Self {
        let bars = spectrum.ranges;
        let out = vec![0; bars];
        let mode = Default::default();
        Self {
            color_index,
            spectrum,
            out,
            filter,
            mode,
        }
    }

    pub fn output(&self) -> &[u32] {
        &self.out
    }
}

impl<T: Filter> Visual for MonoVisualizer<T> {
    fn update(&mut self) {
        self.spectrum.update();
        self.filter.apply(&self.spectrum.amps, &mut self.out);
    }

    fn add_bar(&mut self) {
        self.spectrum.add_range();
        self.out.push(0);
    }

    fn remove_bar(&mut self) {
        self.spectrum.remove_range();
        self.out.pop();
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

    fn color_index(&self) -> usize {
        self.color_index
    }
}
