use std::time::Duration;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::capture::capturer::{Capturer, capture, default_capturer};
use crate::fft::Fft;

pub struct Spectrum {
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

impl Spectrum {
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

    pub async fn init(&mut self) {
        self.capturer
            .init()
            .expect("Error in Capturer Initialization");
        let mut sample_len = self.capturer.buffer_size();
        while sample_len == 0 {
            sample_len = self.capturer.buffer_size();
            std::thread::sleep(Duration::from_millis(100));
        }
        let (tx, rx_from_fft) = mpsc::channel(1);
        let (tx_to_fft, rx) = mpsc::channel(1);
        self.tx = Some(tx);
        self.rx = Some(rx);
        let fft = Fft::new(self.sample_len, tx_to_fft, rx_from_fft);
        fft.init().await;
    }

    // pub fn new_stereo(ranges: usize) -> Self {
    //     let capturer = default_interleaved_capturer();
    //     capturer
    //         .init()
    //         .expect("Error in Interpolated Capturer Initialization");
    //     if capturer.channels() != 2 {
    //         panic!("Attemped stereo playback without 2 channels");
    //     }
    //     let sample_len = capturer.buffer_size();
    //     let amps = vec![0; ranges];
    //     let fft = Fft::new(sample_len / 2);
    //     let scale = DEFAULT_SCALE;
    //     Self {
    //         capturer,
    //         ranges,
    //         amps,
    //         scale,
    //         fft,
    //         sample_len,
    //     }
    // }
    //
    pub fn update(&mut self) {
        // self.fft.place_input(sample.as_slice());
        // let transform = self.fft.transform();
        let Some(tx) = &self.tx else {
            panic!("fft sender not initialized")
        };
        let Some(rx) = self.rx.as_mut() else {
            panic!("fft receiver not initialized")
        };

        let sample = Spectrum::sample();
        if sample.is_empty() {
            return;
        }
        if tx.try_send(sample).is_err() {
            panic!("fft reviecer dropped or full");
        }

        let Ok(transform) = rx.try_recv() else {
            return;
        };

        let transform_len = transform.len(); // same len as sample.len() shrug
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

    pub fn add_range(&mut self) {
        self.ranges += 1;
        self.amps = vec![0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    pub fn remove_range(&mut self) {
        self.ranges -= 1;
        self.amps = vec![0; self.ranges];
        // let len = (self.ranges - 1) * 2;
        // self.capturer = Capturer::new(len);
    }

    pub fn adjust_scale(&mut self, value: f32) {
        if self.scale + value > 0.0 {
            self.scale += value
        } else {
            self.scale = 0.0
        }
    }

    pub fn sample_rate(&self) -> usize {
        self.capturer.rate()
    }

    pub fn channels(&self) -> usize {
        self.capturer.channels()
    }

    pub fn max(&self) -> Option<u32> {
        self.amps.iter().max().copied()
    }
}
