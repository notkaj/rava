use crate::{
    spectrum::{average::AverageSpectrum, spectral::Spectral},
    visualize::visual::{COLORS, DEFAULT_COLOR_INDEX, Mode, Visual},
};
use bounded_vec_deque::BoundedVecDeque;

struct Waterfall {
    spectrum: AverageSpectrum,
    pub out: BoundedVecDeque<Vec<u32>>,
    color_index: usize,
    mode: Mode,
}

impl Default for Waterfall {
    fn default() -> Self {
        let spectrum = AverageSpectrum::default();
        let out = BoundedVecDeque::with_capacity(8, 8);
        let color_index = DEFAULT_COLOR_INDEX;
        let mode = Default::default();
        Self {
            spectrum,
            out,
            color_index,
            mode,
        }
    }
}

impl Waterfall {
    fn new(bound: usize) -> Self {
        let spectrum = AverageSpectrum::default();
        let out = BoundedVecDeque::with_capacity(bound, bound);
        let color_index = DEFAULT_COLOR_INDEX;
        let mode = Default::default();
        Self {
            spectrum,
            out,
            color_index,
            mode,
        }
    }
}

impl Visual for Waterfall {
    fn update(&mut self) {
        self.spectrum.update();
        self.out.push_front(self.spectrum.amps.clone());
    }

    fn add_bar(&mut self) {
        self.out.clear();
        self.spectrum.add_range()
    }

    fn remove_bar(&mut self) {
        self.out.clear();
        self.spectrum.add_range()
    }

    fn increment_scale(&mut self) {
        self.spectrum.adjust_scale(1.0)
    }

    fn decrement_scale(&mut self) {
        self.spectrum.adjust_scale(-1.0)
    }

    fn color(&self) -> ratatui::prelude::Color {
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
