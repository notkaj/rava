use std::cmp;
use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::DEFAULT_QUANT;
use crate::capture::capturer::{Capturer, capture, default_interleaved_capturer};
use crate::fft::{Fft, exchange};
use crate::spectrum::{DEFAULT_RANGE_COUNT, DEFAULT_SCALE, OFFSET, RATIO, Spectrum};
use crate::spectrum::{apply_hann_window, hann_multipliers};

pub struct StereoSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub left_amps: Vec<f32>,
    pub right_amps: Vec<f32>,
    scale: f32, // should be moved out to Visualizer?
    left_tx: Option<Sender<Vec<f32>>>,
    left_rx: Option<Receiver<Vec<f32>>>,
    right_tx: Option<Sender<Vec<f32>>>,
    right_rx: Option<Receiver<Vec<f32>>>,
    pub sample_len: usize,
    multipliers: Vec<f32>,
}

impl Default for StereoSpectrum {
    fn default() -> Self {
        StereoSpectrum::new(DEFAULT_RANGE_COUNT)
    }
}

impl StereoSpectrum {
    pub fn new(ranges: usize) -> Self {
        let capturer = default_interleaved_capturer();
        let left_amps = vec![0.0; ranges];
        let right_amps = vec![0.0; ranges];

        let sample_len = DEFAULT_QUANT;
        let multipliers = vec![0.0; sample_len];

        let scale = DEFAULT_SCALE;
        Self {
            capturer,
            ranges,
            left_amps,
            right_amps,
            scale,
            left_tx: None,
            left_rx: None,
            right_tx: None,
            right_rx: None,
            sample_len,
            multipliers,
        }
    }

    fn sample() -> (Vec<f32>, Vec<f32>) {
        // TODO: handle errors
        let cap = capture().unwrap();
        let len = cap.len() / 2;
        let mut left = vec![0.0; len];
        let mut right = vec![0.0; len];
        for i in 0..len {
            left[i] = cap[i * 2];
            right[i] = cap[i * 2 + 1];
        }
        (left, right)
    }

    pub fn init(&mut self) {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");

        let mut sample_len = self.capturer.buffer_size();
        while sample_len == 0 {
            sample_len = self.capturer.buffer_size();
            std::thread::sleep(Duration::from_millis(100));
        }

        self.multipliers = hann_multipliers(sample_len / 2);

        let (left_tx, left_rx_from_spectrum) = mpsc::channel(1);
        let (left_tx_to_spectrum, left_rx) = mpsc::channel(1);
        let (right_tx, right_rx_from_spectrum) = mpsc::channel(1);
        let (right_tx_to_spectrum, right_rx) = mpsc::channel(1);
        let _ = left_tx.try_send(vec![0.0; sample_len / 2]);
        let _ = right_tx.try_send(vec![0.0; sample_len / 2]);

        self.left_tx = Some(left_tx);
        self.left_rx = Some(left_rx);
        self.right_tx = Some(right_tx);
        self.right_rx = Some(right_rx);

        if self.capturer.channels() != 2 {
            panic!("Attempted stereo playback without 2 channels")
        }

        let left_fft = Fft::new(sample_len / 2, left_tx_to_spectrum, left_rx_from_spectrum);
        let right_fft = Fft::new(sample_len / 2, right_tx_to_spectrum, right_rx_from_spectrum);
        left_fft.init();
        right_fft.init();
    }
}

impl Spectrum for StereoSpectrum {
    fn update(&mut self) {
        let sample_len = DEFAULT_QUANT / 2;

        let (mut left_sample, mut right_sample) = StereoSpectrum::sample();

        let (left_tx, right_tx) = match (self.left_tx.as_ref(), self.right_tx.as_ref()) {
            (Some(ltx), Some(rtx)) => (ltx, rtx),
            _ => panic!("samples count not be transferred: tx not initialized"),
        };

        let (left_rx, right_rx) = match (self.right_rx.as_mut(), self.left_rx.as_mut()) {
            (Some(lrx), Some(rrx)) => (lrx, rrx),
            _ => panic!("transform could not be received: rx not initialized"),
        };

        let left_transform = if left_sample.is_empty() {
            let fake = vec![0.0; sample_len];
            exchange(fake, left_tx, left_rx)
        } else {
            apply_hann_window(&mut left_sample, &self.multipliers);
            exchange(left_sample, left_tx, left_rx)
        };

        let right_transform = if right_sample.is_empty() {
            let fake = vec![0.0; sample_len];
            exchange(fake, right_tx, right_rx)
        } else {
            apply_hann_window(&mut right_sample, &self.multipliers);
            exchange(right_sample, right_tx, right_rx)
        };

        // let len = (transform_len as f32 * RATIO) as usize;
        let range_len = self.range_len();

        for i in 0..self.ranges {
            let start = (i + OFFSET) * range_len;
            let end = start + range_len;
            let left_avg = left_transform[start..end].iter().sum::<f32>() / range_len as f32;
            let right_avg = right_transform[start..end].iter().sum::<f32>() / range_len as f32;
            let left_root = left_avg.sqrt();
            let right_root = right_avg.sqrt();
            self.left_amps[i] = left_root * self.scale;
            self.right_amps[i] = right_root * self.scale;
        }
    }

    fn add_range(&mut self) {
        self.ranges += 1;
        self.left_amps = vec![0.0; self.ranges];
        self.right_amps = vec![0.0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    fn remove_range(&mut self) {
        self.ranges -= 1;
        self.left_amps = vec![0.0; self.ranges];
        self.right_amps = vec![0.0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    fn adjust_scale(&mut self, value: f32) {
        if self.scale + value > 0.0 {
            self.scale += value
        } else {
            self.scale = 0.0
        }
    }

    fn sample_rate(&self) -> usize {
        self.capturer.rate()
    }

    fn channels(&self) -> usize {
        self.capturer.channels()
    }

    fn max_amp(&self) -> Option<u32> {
        let left = self.left_amps.iter().map(|&f| f as u32).max();
        let right = self.right_amps.iter().map(|&f| f as u32).max();
        cmp::max(left, right)
    }

    fn range_len(&self) -> usize {
        let len = (self.sample_len as f32 * RATIO) as usize;
        len / self.ranges
    }
}
