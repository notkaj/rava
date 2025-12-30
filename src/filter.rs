pub trait Filter {
    fn apply(&self, raw: &[u32], out: &mut [u32]);
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
    fn apply(&self, raw: &[u32], out: &mut [u32]) {
        for (i, e) in raw.iter().enumerate() {
            if *e > out[i] + 2 {
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
    fn apply(&self, raw: &[u32], out: &mut [u32]) {
        for (i, e) in raw.iter().enumerate() {
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
