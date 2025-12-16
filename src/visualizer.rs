use ratatui::style::Color;
use std::cmp;

use crate::spectrum::Spectrum;

const DEFAULT_BAR_COUNT: usize = 72;

pub struct Visualizer {
    pub color: Color,
    pub spectrum: Spectrum,
    pub out: Vec<u32>,
}

impl Default for Visualizer {
    fn default() -> Self {
        Self::new(DEFAULT_BAR_COUNT, Color::Green)
    }
}

impl Visualizer {
    pub fn new(bars: usize, color: Color) -> Self {
        let spectrum = Spectrum::new(bars);
        let out = vec![0; bars];
        Self {
            color,
            spectrum,
            out,
        }
    }

    pub fn update(&mut self) {
        self.spectrum.update();
        for (i, e) in self.spectrum.amps.iter().enumerate() {
            let curr = self.out[i];
            // TODO: make the divisor a ratio, make it a const
            let decay = curr.div_ceil(10);
            let new = curr - decay; // this never overflows somehow
            self.out[i] = cmp::max(new, *e);
        }
    }

    // fn add_bar(&mut self) {
    //     self.spectrum.add_range();
    // }
    //
    // fn remove_bar(&mut self) {
    //     self.spectrum.remove_range();
    // }
    //
    // fn color(&mut self, color: Color) {
    //     self.color = color;
    // }
    //

    pub fn bars(&self) -> usize {
        self.spectrum.ranges
    }
}
