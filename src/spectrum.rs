use crate::capture::capturer::{Capturer, capture, default_capturer};
use crate::fft::Fft;
use rand::{Rng, rng};

pub struct Spectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub amps: Vec<u32>,
    fft: Fft,
}

const SKIP: usize = 10;
const PRUNE: usize = 2;

impl Spectrum {
    pub fn new(ranges: usize) -> Self {
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_capturer();
        capturer.init().expect("Error in Capturer Initialization");
        let amps = vec![0; ranges];
        let fft = Fft::default();
        Self {
            capturer,
            ranges,
            amps,
            fft,
        }
    }

    pub fn update(&mut self) {
        let sample = Spectrum::sample();

        if sample.is_empty() || sample.len() != 2048 {
            return;
        }

        self.fft.place_input(sample.as_slice());
        let transform = self.fft.transform();

        let transform_len = transform.len();
        // I'll skip the first 10% of the transform
        let first = transform_len / SKIP;
        // I'll clip off the last 50% of the transform
        let last = first + (transform_len / PRUNE);
        let len = last - first;
        let range_len = len / self.ranges;

        for i in 0..self.ranges {
            let start = i * range_len;
            let end = start + range_len;
            let avg = transform[start..end].iter().sum::<f32>() / range_len as f32;
            self.amps[i] = (avg * 50.0) as u32;
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

    pub fn sample_rate(&self) -> usize {
        self.capturer.rate()
    }

    pub fn channels(&self) -> usize {
        self.capturer.channels()
    }

    // pub fn test_data(ranges: usize) -> Self {
    //     // let amps: Vec<u8> = (0..ranges).map(|_| rng().random_range(50..90)).collect();
    //     let capturer = Capturer::default();
    //     let amps = vec![0; ranges];
    //     let fft = Fft::default();
    //     Self {
    //         capturer,
    //         ranges,
    //         amps,
    //         fft,
    //     }
    // }

    pub fn test_sample(&self) -> Vec<u8> {
        let amps: Vec<u8> = (0..self.ranges)
            .map(|_| rng().random_range(10..70))
            .collect();
        amps
    }
}
