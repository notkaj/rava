use ratatui::style::Color;

use crate::spectrum::Spectrum;

const DEFAULT_BAR_COUNT: usize = 48;

pub struct Visualizer {
    pub color: Color,
    pub spectrum: Spectrum,
}

impl Default for Visualizer {
    fn default() -> Self {
        Self::new(DEFAULT_BAR_COUNT)
    }
}

impl Visualizer {
    pub fn new(bars: usize) -> Self {
        let color = Color::Green;
        let spectrum = Spectrum::new(bars);
        Self { color, spectrum }
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
