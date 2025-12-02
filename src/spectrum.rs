use crate::capture::capturer::{Capturer, default_capturer};
use crate::fft::Fft;
use rand::{Rng, rng};

pub struct Spectrum {
    capturer: Box<dyn Capturer>,
    pub ranges: usize,
    pub amps: Vec<u32>,
    fft: Fft,
}

impl Default for Spectrum {
    fn default() -> Self {
        let capturer = default_capturer();
        let ranges = 24;
        let amps = Vec::new();
        Spectrum {
            capturer,
            ranges,
            amps,
            fft: Default::default(),
        }
    }
}

impl Spectrum {
    pub fn new(ranges: usize /* capturer: impl Capturer*/) -> Self {
        // let len = (ranges - 1) * 2;
        // eprintln!("Initializeing Audio Stream");
        let capturer = default_capturer();
        capturer.init().expect("Capturer Initialization failed");
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
        let sample = self.sample();
        // print!("samples: ");
        // println!("{:?}", sample);
        self.fft.place_input(sample.as_slice());
        let amps = self.fft.transform();
        // print!("amps: ");
        // println!("{:?}", amps);
        let length = amps.len();

        let range_length = length / self.ranges;

        for i in 0..self.ranges {
            let start = i * range_length;
            let end = start + range_length;
            let avg = amps[start..end].iter().sum::<f32>() / range_length as f32;
            self.amps[i] = avg as u32;
        }

        // println!("avgs: ");
        // println!("{:?}", self.amps);
    }

    pub fn sample(&self) -> Vec<f32> {
        let res = self.capturer.capture();
        res.unwrap().iter().map(|a| a.round()).collect()
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
