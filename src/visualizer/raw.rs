use crate::{
    capture::capturer::{Capturer, capture, default_capturer},
    visualizer::{COLORS, Mode, Visualizer},
};

pub struct RawMonoVisualizer {
    capturer: Box<dyn Capturer>,
    out: Vec<f64>,
    pub mode: Mode,
    pub color_index: usize,
    pub scale: f32,
}

impl Default for RawMonoVisualizer {
    fn default() -> Self {
        RawMonoVisualizer {
            capturer: default_capturer(),
            out: Vec::new(),
            mode: Default::default(),
            color_index: Default::default(),
            scale: 10.,
        }
    }
}

impl RawMonoVisualizer {
    pub fn new(points: usize) -> Self {
        let out = vec![0.; points];
        RawMonoVisualizer {
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
}

impl Visualizer for RawMonoVisualizer {
    fn update(&mut self) {
        let amps = capture();

        if let Ok(a) = amps {
            let n = a.len();
            let ranges = self.out.len();
            let range_len = n / ranges;

            for i in 0..ranges {
                let start = i * range_len;
                let end = start + range_len;
                let sum = a[start..end].iter().sum::<f32>();
                let avg = sum / range_len as f32;
                self.out[i] = f64::from(avg * self.scale);
            }
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
