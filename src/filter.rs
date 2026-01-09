use std::cell::RefCell;

pub trait Filter {
    fn apply(&self, input: &[u32], out: &mut [u32]);
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
    fn apply(&self, input: &[u32], out: &mut [u32]) {
        for (i, e) in input.iter().enumerate() {
            if *e > out[i] {
                let diff = *e - out[i];
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

const DEFAULT_PEAK_DUR_TICKS: u8 = 5;

pub struct ExperimentalFilter {
    rate_of_decay: f32,
    peak_dur_ticks: u8,
    ticks: RefCell<Vec<u8>>,
}

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
    fn apply(&self, input: &[u32], out: &mut [u32]) {
        for (i, e) in input.iter().enumerate() {
            let entry = *e;
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
    fn apply(&self, input: &[u32], out: &mut [u32]) {
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
    fn apply(&self, raw: &[u32], out: &mut [u32]) {
        out.copy_from_slice(raw);
    }
}
