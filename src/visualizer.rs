use ratatui::style::Color;
use std::cmp;

use crate::filter::{Filter, NormalFilter};
use crate::spectrum::Spectrum;

const DEFAULT_BAR_COUNT: usize = 72;
const DEFAULT_RATE_OF_DECAY: f32 = 0.1; // i think this should be removed and placed in filter
const DEFAULT_COLOR: Color = Color::Green;

pub struct Visualizer<T: Filter> {
    pub color: Color,
    spectrum: Spectrum,
    pub out: Vec<u32>,
    filter: T,
}

impl Default for Visualizer<NormalFilter> {
    fn default() -> Self {
        Self::new(DEFAULT_BAR_COUNT, DEFAULT_COLOR, Default::default())
    }
}

impl<T: Filter> Visualizer<T> {
    pub fn new(bars: usize, color: Color, filter: T) -> Self {
        let spectrum = Spectrum::new(bars);
        let out = vec![0; bars];
        Self {
            color,
            spectrum,
            out,
            filter,
        }
    }

    pub fn update(&mut self) {
        self.spectrum.update();
        for (i, e) in self.spectrum.amps.iter().enumerate() {
            let curr = self.out[i];
            let decay = (curr as f32 * DEFAULT_RATE_OF_DECAY).ceil() as u32;
            // let new = curr.saturating_sub(decay);
            let new = curr - decay; // this doesn't overflow somehow
            self.out[i] = cmp::max(new, *e);
        }
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

    // fn color(&mut self, color: Color) {
    //     self.color = color;
    // }
    //

    pub fn bars(&self) -> usize {
        self.spectrum.ranges
    }
}
