use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::capturer::{Capturer, capture, default_capturer};
use crate::fft::Fft;
use crate::spectrum::spectral::Spectral;

pub struct AverageSpectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub amps: Vec<u32>,
    scale: f32, // should be moved out to Visualizer?
    tx: Option<Sender<Vec<f32>>>,
    rx: Option<Receiver<Vec<f32>>>,
    pub sample_len: usize,
}

const RATIO: f32 = 0.10;
const OFFSET: usize = 0;
const DEFAULT_SCALE: f32 = 48.0;

impl AverageSpectrum {
    pub fn new(ranges: usize) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_capturer();
        let amps = vec![0; ranges];
        let tx = None;
        let rx = None;
        let sample_len = 2048;
        let scale = DEFAULT_SCALE;
        Self {
            capturer,
            ranges,
            amps,
            scale,
            tx,
            rx,
            sample_len,
        }
    }

    fn sample() -> Vec<f32> {
        // TODO: handle errors
        capture().unwrap()
    }
}

impl Spectral for AverageSpectrum {
    fn init(&mut self) {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");
        let mut sample_len = self.capturer.buffer_size();
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

    fn update(&mut self) {
        // self.fft.place_input(sample.as_slice());
        // let transform = self.fft.transform();
        let Some(tx) = &self.tx else {
            panic!("fft sender not initialized")
        };
        let Some(rx) = self.rx.as_mut() else {
            panic!("fft receiver not initialized")
        };

        let Ok(transform) = rx.try_recv() else {
            // TODO: recover form this
            panic!("ui and fft out of sync");
        };

        let transform_len = transform.len();
        let sample = AverageSpectrum::sample();
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
