use std::cell::RefCell;

use crate::event::TICK_FPS;

pub trait Filter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]);
}

pub struct NormalFilter {
    rate_of_decay: f32,
    rate_of_increase: f32,
}

const DEFAULT_RATE_OF_DECAY: f32 = 0.05;
const DEFAULT_RATE_OF_INCREASE: f32 = 0.75;

impl Default for NormalFilter {
    fn default() -> Self {
        Self::new(DEFAULT_RATE_OF_DECAY, DEFAULT_RATE_OF_INCREASE)
    }
}

impl NormalFilter {
    fn new(rate_of_decay: f32, rate_of_increase: f32) -> Self {
        Self {
            rate_of_decay,
            rate_of_increase,
        }
    }
}

impl Filter for NormalFilter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]) {
        for (i, e) in input.iter().enumerate() {
            let e = *e as u32;
            if e > out[i] {
                let diff = e - out[i];
                out[i] += (diff as f32 * self.rate_of_increase) as u32;
                // out[i] = *e;
            } else {
                let curr = out[i];
                let decay = (curr as f32 * self.rate_of_decay).ceil() as u32;
                // let new = curr.saturating_sub(decay);
                out[i] = curr - decay; // this doesn't overflow somehow
            }
        }
    }
}

const DEFAULT_NOISE_REDUCT: f32 = 0.77;

pub struct CavaFilter {
    curr_peaks: Vec<f32>,
    fall_vals: Vec<f32>,
    noise_reduct: f32,
    prev_amps: Vec<f32>,
    post_int_vals: Vec<f32>,
}

impl CavaFilter {
    fn new(len: usize, noise_reduct: f32) -> Self {
        let curr_peaks = vec![0.0; len];
        let fall_vals = vec![0.0; len];
        let prev_amps = vec![0.0; len];
        let post_int_vals = vec![0.0; len];
        Self {
            curr_peaks,
            fall_vals,
            noise_reduct,
            prev_amps,
            post_int_vals,
        }
    }

    pub fn from_len(len: usize) -> Self {
        Self::new(len, DEFAULT_NOISE_REDUCT)
    }

    pub fn resize(&mut self, len: usize) {
        self.curr_peaks.resize(len, 0.0);
        self.fall_vals.resize(len, 0.0);
        self.prev_amps.resize(len, 0.0);
        self.post_int_vals.resize(len, 0.0);
    }
}

impl Filter for CavaFilter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]) {
        let len = input.len();
        if self.curr_peaks.len() != len {
            self.resize(len)
        }
        // let overshoot = 0;
        let gravity_mod = ((60.0 / TICK_FPS as f32).powf(2.5) * 1.54 / self.noise_reduct).max(1.0);

        for (i, &amp) in input.iter().enumerate() {
            let prev = self.prev_amps[i];
            let mut res = amp;
            if amp < prev && self.noise_reduct > 0.1 {
                res = self.curr_peaks[i] * (1.0 - (self.fall_vals[i].powi(2) * gravity_mod));
                self.fall_vals[i] -= 0.028;
            } else {
                self.curr_peaks[i] = amp;
                self.fall_vals[i] = 0.0;
            }
            self.prev_amps[i] = res;

            // process integral smoothing
            let post_int = self.post_int_vals[i] * self.noise_reduct + res;
            self.post_int_vals[i] = post_int;
            out[i] = post_int as u32;
        }
    }
}

const DEFAULT_PEAK_DUR_TICKS: u8 = 5;

pub struct ExperimentalFilter {
    rate_of_decay: f32,
    peak_dur_ticks: u8,
    ticks: RefCell<Vec<u8>>,
}

#[allow(dead_code)]
impl ExperimentalFilter {
    pub fn new(len: usize, rate_of_decay: f32, peak_dur_ticks: u8) -> Self {
        // TODO: this vec is never adjusted, so if the number of bars is
        // ever increased during runtime, the program WILL panic
        let ticks = RefCell::new(vec![0; len]);
        Self {
            rate_of_decay,
            peak_dur_ticks,
            ticks,
        }
    }

    pub fn new_default(len: usize) -> Self {
        ExperimentalFilter::new(len, DEFAULT_RATE_OF_DECAY, DEFAULT_PEAK_DUR_TICKS)
    }
}

impl Filter for ExperimentalFilter {
    fn apply(&mut self, input: &[f32], out: &mut [u32]) {
        for (i, e) in input.iter().enumerate() {
            let entry = *e as u32;
            let tick = self.ticks.borrow()[i];
            if tick == 0 && entry > out[i] {
                // let diff = entry - out[i];
                // out[i] += (diff as f32 * self.rate_of_increase) as u32;
                out[i] = entry;
                self.ticks.borrow_mut()[i] = self.peak_dur_ticks;
            } else {
                // i don't think this needs to be saturated
                self.ticks.borrow_mut()[i] = tick.saturating_sub(1);
                let curr = out[i];
                let decay = (curr as f32 * self.rate_of_decay).ceil() as u32;
                out[i] = curr - decay;
            }
        }
    }
}

pub struct SmoothFilter {
    rate_of_diff: f32,
}

const DEFAULT_DIFF_RATE: f32 = 0.50;

impl Default for SmoothFilter {
    fn default() -> Self {
        Self::new(DEFAULT_DIFF_RATE)
    }
}

#[allow(dead_code)]
impl SmoothFilter {
    fn new(rate_of_diff: f32) -> Self {
        Self { rate_of_diff }
    }
}

impl Filter for SmoothFilter {
    // i haven't tested this at all
    fn apply(&mut self, input: &[f32], out: &mut [u32]) {
        for (i, e) in input.iter().enumerate() {
            let diff: i32 = *e as i32 - out[i] as i32;
            let change = diff as f32 * self.rate_of_diff;
            let new = out[i] as i32 + change as i32;
            out[i] = new as u32;
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct RawFilter;
impl Filter for RawFilter {
    fn apply(&mut self, raw: &[f32], out: &mut [u32]) {
        for (i, e) in raw.iter().enumerate() {
            out[i] = *e as u32;
        }
    }
}
