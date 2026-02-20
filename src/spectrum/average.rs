use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::DEFAULT_QUANT;
use crate::capture::capturer::{Capturer, capture, default_capturer};
use crate::fft::Fft;
use crate::spectrum::spectral::{DEFAULT_RANGE_COUNT, DEFAULT_SCALE, OFFSET, RATIO, Spectral};

pub struct AverageSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub amps: Vec<u32>,
    scale: f32, // should be moved out to Visualizer?
    tx: Sender<Vec<f32>>,
    rx: Receiver<Vec<f32>>,
    pub sample_len: usize,
}

impl Default for AverageSpectrum {
    fn default() -> Self {
        AverageSpectrum::new(DEFAULT_RANGE_COUNT)
    }
}

impl AverageSpectrum {
    pub fn new(ranges: usize) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_capturer();
        let amps = vec![0; ranges];

        let (tx, rx_from_spectrum) = mpsc::channel(1);
        let (tx_to_spectrum, rx) = mpsc::channel(1);
        let sample_len = DEFAULT_QUANT;
        let _ = tx.try_send(vec![0.0; sample_len]);

        let scale = DEFAULT_SCALE;
        let res = Self {
            capturer,
            ranges,
            amps,
            scale,
            tx,
            rx,
            sample_len,
        };
        res.init(rx_from_spectrum, tx_to_spectrum)
    }

    fn init(self, rx_from_spectrum: Receiver<Vec<f32>>, tx_to_spectrum: Sender<Vec<f32>>) -> Self {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");
        let mut sample_len = self.capturer.buffer_size();
        // TODO: this is stupid
        while sample_len == 0 {
            sample_len = self.capturer.buffer_size();
            std::thread::sleep(Duration::from_millis(100));
        }
        let fft = Fft::new(self.sample_len, tx_to_spectrum, rx_from_spectrum);
        fft.init();
        self
    }

    fn sample() -> Vec<f32> {
        // TODO: handle errors
        capture().unwrap()
    }
}

impl Spectral for AverageSpectrum {
    fn update(&mut self) {
        let Ok(transform) = self.rx.try_recv() else {
            // TODO: recover form this
            panic!("ui and fft out of sync");
        };

        let transform_len = transform.len();
        let sample = AverageSpectrum::sample();
        if sample.is_empty() {
            // maybe handle if this errors
            let _ = self.tx.try_send(vec![0.0; transform_len]);
            return;
        }

        if self.tx.try_send(sample).is_err() {
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

    fn max_amp(&self) -> Option<u32> {
        self.amps.iter().max().copied()
    }
}
