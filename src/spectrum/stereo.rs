use std::cmp;
use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::capturer::{Capturer, capture, default_interleaved_capturer};
use crate::fft::Fft;
use crate::spectrum::spectral::{DEFAULT_RANGE_COUNT, Spectral};

pub struct StereoSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub left_amps: Vec<u32>,
    pub right_amps: Vec<u32>,
    scale: f32, // should be moved out to Visualizer?
    left_tx: Sender<Vec<f32>>,
    left_rx: Receiver<Vec<f32>>,
    right_tx: Sender<Vec<f32>>,
    right_rx: Receiver<Vec<f32>>,
    pub sample_len: usize,
}

const RATIO: f32 = 0.10;
const OFFSET: usize = 0;
const DEFAULT_SCALE: f32 = 48.0;

impl Default for StereoSpectrum {
    fn default() -> Self {
        StereoSpectrum::new(DEFAULT_RANGE_COUNT)
    }
}

impl StereoSpectrum {
    pub fn new(ranges: usize) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_interleaved_capturer();
        let left_amps = vec![0; ranges];
        let right_amps = vec![0; ranges];

        let (left_tx, left_rx_from_spectrum) = mpsc::channel(1);
        let (left_tx_to_spectrum, left_rx) = mpsc::channel(1);
        let (right_tx, right_rx_from_spectrum) = mpsc::channel(1);
        let (right_tx_to_spectrum, right_rx) = mpsc::channel(1);
        let sample_len = 2048;
        let _ = left_tx.try_send(vec![0.0; sample_len]);
        let _ = right_tx.try_send(vec![0.0; sample_len]);

        let scale = DEFAULT_SCALE;
        let res = Self {
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
        };
        res.init(
            left_rx_from_spectrum,
            left_tx_to_spectrum,
            right_rx_from_spectrum,
            right_tx_to_spectrum,
        )
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

    fn init(
        self,
        left_rx_from_spectrum: Receiver<Vec<f32>>,
        left_tx_to_spectrum: Sender<Vec<f32>>,
        right_rx_from_spectrum: Receiver<Vec<f32>>,
        right_tx_to_spectrum: Sender<Vec<f32>>,
    ) -> Self {
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

        let left_fft = Fft::new(sample_len / 2, left_tx_to_spectrum, left_rx_from_spectrum);
        let right_fft = Fft::new(sample_len / 2, right_tx_to_spectrum, right_rx_from_spectrum);
        left_fft.init();
        right_fft.init();
        self
    }
}

impl Spectral for StereoSpectrum {
    fn update(&mut self) {
        let Ok(left_transform) = self.left_rx.try_recv() else {
            // TODO: recover form this
            panic!("ui and fft out of sync");
        };

        let Ok(right_transform) = self.right_rx.try_recv() else {
            panic!("ui and fft out of sync");
        };

        let transform_len = left_transform.len();
        let (left_sample, right_sample) = StereoSpectrum::sample();

        if left_sample.is_empty() {
            // maybe handle if this errors
            let _ = self.left_tx.try_send(vec![0.0; transform_len]);
            return;
        }

        if right_sample.is_empty() {
            let _ = self.right_tx.try_send(vec![0.0; transform_len]);
            return;
        }

        if self.left_tx.try_send(left_sample).is_err() {
            panic!("fft reviecer dropped or full");
        }

        if self.right_tx.try_send(right_sample).is_err() {
            panic!("fft receiver dropped or full");
        }

        let len = (transform_len as f32 * RATIO) as usize;
        let range_len = len / self.ranges;

        for i in 0..self.ranges {
            let start = (i + OFFSET) * range_len;
            let end = start + range_len;
            let left_avg = left_transform[start..end].iter().sum::<f32>() / range_len as f32;
            let right_avg = right_transform[start..end].iter().sum::<f32>() / range_len as f32;
            let left_root = left_avg.sqrt();
            let right_root = right_avg.sqrt();
            self.left_amps[i] = (left_root * self.scale) as u32;
            self.right_amps[i] = (right_root * self.scale) as u32;
        }
    }

    fn add_range(&mut self) {
        self.ranges += 1;
        self.left_amps = vec![0; self.ranges];
        self.right_amps = vec![0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    fn remove_range(&mut self) {
        self.ranges -= 1;
        self.left_amps = vec![0; self.ranges];
        self.right_amps = vec![0; self.ranges];
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
        let left = self.left_amps.iter().max().copied();
        let right = self.right_amps.iter().max().copied();
        cmp::max(left, right)
    }
}
