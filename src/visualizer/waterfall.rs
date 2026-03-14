use std::time::{Duration, Instant};

use super::{COLORS, DEFAULT_COLOR_INDEX, Mode, Visualizer};
use crate::spectrum::{Spectrum, average::AverageSpectrum};
use bounded_vec_deque::BoundedVecDeque;

const DEFAULT_TICK_RATE: Duration = Duration::from_millis(100 / 3);

pub struct Waterfall {
    spectrum: AverageSpectrum,
    pub out: BoundedVecDeque<Vec<f64>>,
    color_index: usize,
    mode: Mode,
    tick_rate: Duration,
    last_tick: Instant,
}

impl Default for Waterfall {
    fn default() -> Self {
        let spectrum = AverageSpectrum::new(24, 10.0);
        let mut out = BoundedVecDeque::new(90);
        out.push_front(vec![0.0; spectrum.ranges]);
        let color_index = DEFAULT_COLOR_INDEX;
        let tick_rate = DEFAULT_TICK_RATE;
        let mode = Default::default();
        let last_tick = Instant::now();
        Self {
            spectrum,
            out,
            color_index,
            mode,
            tick_rate,
            last_tick,
        }
    }
}

impl Waterfall {
    #[allow(dead_code)]
    pub fn new(curves: usize, points: usize, scale: f32) -> Self {
        let spectrum = AverageSpectrum::new(points, scale);
        let mut out = BoundedVecDeque::new(curves);
        out.push_front(vec![0.0; points]);
        Self {
            spectrum,
            out,
            ..Default::default()
        }
    }

    pub fn init(&mut self) {
        self.spectrum.init();
    }
}

impl Visualizer for Waterfall {
    fn update(&mut self) {
        if self.last_tick.elapsed() < self.tick_rate {
            return;
        }
        self.last_tick = Instant::now();
        self.spectrum.update();
        self.out
            .push_front(self.spectrum.amps.iter().map(|&u| u as f64).collect());
    }

    fn add_bar(&mut self) {
        // self.out.clear();
        self.spectrum.add_range()
    }

    fn remove_bar(&mut self) {
        // self.out.clear();
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
