use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::capturer::{Capturer, capture, default_interleaved_capturer};
use crate::fft::Fft;
use crate::spectrum::spectrum::Spectrum;

pub struct AverageSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub left_amps: Vec<u32>,
    pub right_amps: Vec<u32>,
    scale: f32, // should be moved out to Visualizer?
    left_tx: Option<Sender<Vec<f32>>>,
    left_rx: Option<Receiver<Vec<f32>>>,
    right_tx: Option<Sender<Vec<f32>>>,
    right_rx: Option<Receiver<Vec<f32>>>,
    pub sample_len: usize,
}

const RATIO: f32 = 0.10;
const OFFSET: usize = 0;
const DEFAULT_SCALE: f32 = 48.0;

impl AverageSpectrum {
    pub fn new(ranges: usize) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_interleaved_capturer();
        let left_amps = vec![0; ranges];
        let right_amps = vec![0; ranges];
        let left_tx = None;
        let left_rx = None;
        let right_rx = None;
        let right_tx = None;
        let sample_len = 2048;
        let scale = DEFAULT_SCALE;
        Self {
            capturer,
            ranges,
            left_amps,
            right_amps,
            scale,
            left_tx,
            left_rx,
            right_tx,
            right_rx,
            sample_len,
        }
    }
}
impl Spectrum for AverageSpectrum {
    fn init(&mut self) {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");
        let mut sample_len = self.capturer.buffer_size();
        while sample_len == 0 {
            sample_len = self.capturer.buffer_size();
            std::thread::sleep(Duration::from_millis(100));
        }
        if self.capturer.channels() != 2 {
            panic!("Attempted stereo playback without 2 channels")
        }

        let (left_tx, left_rx_from_spectrum) = mpsc::channel(1);
        let (left_tx_to_spectrum, left_rx) = mpsc::channel(1);
        let (right_tx, right_rx_from_spectrum) = mpsc::channel(1);
        let (right_tx_to_spectrum, right_rx) = mpsc::channel(1);
        let _ = left_tx.try_send(vec![0.0; sample_len]);
        let _ = right_tx.try_send(vec![0.0; sample_len]);
        self.left_tx = Some(left_tx);
        self.left_rx = Some(left_rx);
        self.right_tx = Some(right_tx);
        self.right_rx = Some(right_rx);
        let left_fft = Fft::new(
            self.sample_len / 2,
            left_tx_to_spectrum,
            left_rx_from_spectrum,
        );
        let right_fft = Fft::new(
            self.sample_len / 2,
            right_tx_to_spectrum,
            right_rx_from_spectrum,
        );
        left_fft.init();
        right_fft.init();
    }

    fn update(&mut self) {
        // self.fft.place_input(sample.as_slice());
        // let transform = self.fft.transform();
        let Some(left_tx) = &self.left_tx else {
            panic!("fft sender not initialized")
        };
        let Some(left_rx) = self.left_rx.as_mut() else {
            panic!("fft receiver not initialized")
        };

        let Ok(left_transform) = left_rx.try_recv() else {
            // TODO: recover form this
            panic!("ui and fft out of sync");
        };

        let Some(right_tx) = &self.right_tx else {
            panic!("fft sender not initialized");
        };

        let Some(right_rx) = &self.right_rx else {
            panic!("fft receiver not initialized")
        };

        let Ok(right_transform) = right_rx.try_recv() else {
            panic!("ui and fft out of sync");
        };

        let transform_len = left_transform.len();
        let sample = Spectrum::sample();
        if sample.is_empty() {
            // maybe handle if this errors
            let _ = tx.try_send(vec![0.0; transform_len]);
            return;
        }

        if tx.try_send(sample).is_err() {
            panic!("fft reviecer dropped or full");
        }

        let len = (transform_len as f32 * RATIO) as usize;
        let range_len = len / self.ranges;

        for i in 0..self.ranges {
            let start = (i + OFFSET) * range_len;
            let end = start + range_len;
            let avg = transform[start..end].iter().sum::<f32>() / range_len as f32;
            let root = avg.sqrt();
            self.amps[i] = (root * self.scale) as u32;
        }
    }

    fn sample() -> Vec<f32> {
        // TODO: handle errors
        capture().unwrap()
    }

    fn add_range(&mut self) {
        self.ranges += 1;
        self.amps = vec![0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    fn remove_range(&mut self) {
        self.ranges -= 1;
        self.amps = vec![0; self.ranges];
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

    fn max(&self) -> Option<u32> {
        self.amps.iter().max().copied()
    }
}
