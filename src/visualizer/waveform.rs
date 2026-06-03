use std::cmp::{max, min};

use crate::{
    capture::capturer::{Capturer, capture, default_capturer},
    visualizer::{COLORS, Mode, Visualizer},
};

const CROSSING_TRIGGER: bool = true;
const HORIZONTAL_TRIGGER_POSITION: f32 = 0.5;
const TRIGGER_LEVEL: f32 = 0.;

pub struct MonoVisualizer {
    capturer: Box<dyn Capturer>,
    out: Vec<f64>,
    pub mode: Mode,
    pub color_index: usize,
    pub scale: f32,
}

impl Default for MonoVisualizer {
    fn default() -> Self {
        MonoVisualizer {
            capturer: default_capturer(),
            out: Vec::new(),
            mode: Default::default(),
            color_index: Default::default(),
            scale: 10.,
        }
    }
}

impl MonoVisualizer {
    pub fn new(points: usize) -> Self {
        let out = vec![0.; points];
        MonoVisualizer {
            out,
            ..Default::default()
        }
    }

    pub fn init(&mut self) {
        self.capturer.init().expect("Error initializing Capturer");
    }

    pub fn output(&self) -> &[f64] {
        &self.out
    }

    fn trigger(sample: &[f32], trigger_level: f32, htp: f32) -> Vec<f32> {
        let n = sample.len();
        let target_index = (n as f32 * htp) as usize;
        let mut trigger_index = target_index;
        for i in target_index..n.saturating_sub(1) {
            if sample[i] < trigger_level && sample[i + 1] >= trigger_level {
                trigger_index = i;
                break;
            }
        }

        let delta_index = trigger_index - target_index;

        let mut res = vec![0.; n];
        for i in 0..n - delta_index {
            res[i] = sample[i + delta_index];
        }
        res
    }
}

impl Visualizer for MonoVisualizer {
    fn update(&mut self) {
        let result = capture();

        let Ok(amps) = result else {
            return;
        };

        let content = if CROSSING_TRIGGER {
            &MonoVisualizer::trigger(&amps, TRIGGER_LEVEL, HORIZONTAL_TRIGGER_POSITION)
        } else {
            &amps
        };
        // find zero-crossing
        let n = content.len();

        let ranges = self.out.len();
        let range_len = max(1, n / ranges);

        for i in 0..ranges {
            let start = i * range_len;
            let end = min(n, start + range_len);
            if start >= n {
                break;
            }
            let sum = content[start..end].iter().sum::<f32>();
            let len = (end - start) as f32;
            let avg = if len > 0. { sum / len } else { 0. };
            self.out[i] = f64::from(avg * self.scale);
        }
    }

    fn add_bar(&mut self) {
        let len = self.out.len() + 1;
        self.out = vec![0.; len];
    }

    fn remove_bar(&mut self) {
        let len = self.out.len().saturating_sub(1);
        self.out = vec![0.; len];
    }

    fn increment_scale(&mut self) {
        self.scale += 1.;
    }

    fn decrement_scale(&mut self) {
        self.scale -= 1.;
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
        self.capturer.rate()
    }

    fn channels(&self) -> usize {
        self.capturer.channels()
    }

    fn sample_len(&self) -> usize {
        self.capturer.buffer_size()
    }
    fn bars(&self) -> usize {
        self.out.len()
    }

    fn input_max(&self) -> u32 {
        // self.out.iter().copied().max().unwrap_or_default()
        self.out
            .iter()
            .copied()
            .reduce(f64::max)
            .unwrap_or_default() as u32
    }

    fn color_index(&self) -> usize {
        self.color_index
    }

    fn get_mode(&self) -> Mode {
        self.mode
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
}
