use crate::{
    capture::capturer::{Capturer, capture, default_capturer},
    visualizer::{COLORS, Mode, Visualizer},
};

pub struct RawMono {
    capturer: Box<dyn Capturer>,
    out: Vec<u32>,
    pub mode: Mode,
    pub color_index: usize,
    pub scale: usize,
}

impl Default for RawMono {
    fn default() -> Self {
        RawMono {
            capturer: default_capturer(),
            out: Vec::new(),
            mode: Default::default(),
            color_index: Default::default(),
            scale: 1,
        }
    }
}

impl RawMono {
    fn new(points: usize) -> Self {
        let out = vec![0; points];
        RawMono {
            out,
            ..Default::default()
        }
    }

    fn init(&mut self) {
        self.capturer.init().expect("Error initializing Capturer");
    }

    pub fn output(&self) -> &[u32] {
        &self.out
    }
}

impl Visualizer for RawMono {
    fn update(&mut self) {
        let amps = capture();

        if let Ok(a) = amps {
            for (i, e) in a.iter().enumerate() {
                self.out[i] = *e as u32;
            }
        }
    }

    fn add_bar(&mut self) {
        let len = self.out.len() + 1;
        self.out = vec![0; len];
    }

    fn remove_bar(&mut self) {
        let len = self.out.len().saturating_sub(1);
        self.out = vec![0; len];
    }

    fn increment_scale(&mut self) {
        self.scale += 1;
    }

    fn decrement_scale(&mut self) {
        self.scale -= 1;
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
        self.out.iter().copied().max().unwrap_or_default()
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
