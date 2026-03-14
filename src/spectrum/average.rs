use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use super::{DEFAULT_RANGE_COUNT, DEFAULT_SCALE, OFFSET, RATIO, Spectrum};
use super::{apply_hann_window, hann_multipliers};
use crate::capture::DEFAULT_QUANT;
use crate::capture::capturer::{Capturer, capture, default_capturer};
use crate::fft::Fft;

pub struct AverageSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub amps: Vec<f32>,
    scale: f32, // should be moved out to Visualizer?
    tx: Option<Sender<Vec<f32>>>,
    rx: Option<Receiver<Vec<f32>>>,
    pub sample_len: usize,
    multipliers: Vec<f32>,
}

impl Default for AverageSpectrum {
    fn default() -> Self {
        AverageSpectrum::new(DEFAULT_RANGE_COUNT, DEFAULT_SCALE)
    }
}

impl AverageSpectrum {
    pub fn new(ranges: usize, scale: f32) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_capturer();
        let amps = vec![0.0; ranges];

        let sample_len = DEFAULT_QUANT;

        let multipliers = hann_multipliers(sample_len);

        Self {
            capturer,
            ranges,
            amps,
            scale,
            tx: None,
            rx: None,
            sample_len,
            multipliers,
        }
    }

    pub fn init(&mut self) {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");

        let mut sample_len = self.capturer.buffer_size();
        // TODO: this is stupid
        while sample_len == 0 {
            sample_len = self.capturer.buffer_size();
            std::thread::sleep(Duration::from_millis(100));
        }

        let (tx, rx_from_spectrum) = mpsc::channel(1);
        let (tx_to_spectrum, rx) = mpsc::channel(1);
        let _ = tx.try_send(vec![0.0; sample_len]);
        self.tx = Some(tx);
        self.rx = Some(rx);

        let fft = Fft::new(self.sample_len, tx_to_spectrum, rx_from_spectrum);
        fft.init();
    }

    fn sample() -> Vec<f32> {
        // TODO: handle errors
        capture().unwrap()
    }
}

impl Spectrum for AverageSpectrum {
    fn update(&mut self) {
        let Ok(transform) = self.rx.as_mut().unwrap().try_recv() else {
            // TODO: recover from this
            panic!("ui and fft out of sync");
        };

        let transform_len = transform.len();
        let mut sample = AverageSpectrum::sample();

        if sample.is_empty() {
            // maybe handle if this errors
            let _ = self.tx.as_ref().unwrap().try_send(vec![0.0; transform_len]);
            return;
        }

        apply_hann_window(&mut sample, &self.multipliers);

        if self.tx.as_ref().unwrap().try_send(sample).is_err() {
            panic!("fft reviecer dropped or full");
        }

        // let len = (transform_len as f32 * RATIO) as usize;
        let range_len = self.range_len();

        for i in 0..self.ranges {
            let start = (i + OFFSET) * range_len;
            let end = start + range_len;
            let avg = transform[start..end].iter().sum::<f32>() / range_len as f32;
            let root = avg.sqrt();
            self.amps[i] = root * self.scale;
        }
    }

    fn add_range(&mut self) {
        self.ranges += 1;
        if self.range_len() == 0 {
            self.ranges -= 1;
            return;
        }
        self.amps = vec![0.0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    fn remove_range(&mut self) {
        self.ranges -= 1;
        self.amps = vec![0.0; self.ranges];
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
        self.amps.iter().map(|&n| n as u32).max()
    }

    fn range_len(&self) -> usize {
        let len = (self.sample_len as f32 * RATIO) as usize;
        len / self.ranges
    }
}
