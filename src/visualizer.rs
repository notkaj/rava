use ratatui::style::Color;

use crate::filter::Filter;
use crate::spectrum::Spectrum;

const DEFAULT_BAR_COUNT: usize = 72;
const DEFAULT_COLOR: Color = Color::Blue;

pub struct Visualizer<T: Filter> {
    pub color: Color,
    spectrum: Spectrum,
    out: Vec<u32>,
    filter: T,
    pub mode: Mode,
}

impl<T: Filter + Default> Default for Visualizer<T> {
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
            mode: Default::default(),
        }
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

    // fn color(&mut self, color: Color) {
    //     self.color = color;
    // }

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
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Default,
    ColorPick,
    ShowStats,
    ShowKeys,
}
