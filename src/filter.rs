use std::cmp;

pub trait Filter {
    fn apply(&self, raw: &[u32], out: &mut [u32]);
}

pub struct NormalFilter {
    rate_of_decay: f32,
}

const DEFAULT_RATE_OF_DECAY: f32 = 0.05;

impl Default for NormalFilter {
    fn default() -> Self {
        Self::new(DEFAULT_RATE_OF_DECAY)
    }
}

impl NormalFilter {
    fn new(rate_of_decay: f32) -> Self {
        Self { rate_of_decay }
    }
}

impl Filter for NormalFilter {
    fn apply(&self, raw: &[u32], out: &mut [u32]) {
        for (i, e) in raw.iter().enumerate() {
            if *e > out[i] {
                let diff = *e - out[i];
                out[i] += diff - (diff / 4);
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

#[allow(dead_code)]
pub struct RawFilter;

impl Filter for RawFilter {
    fn apply(&self, raw: &[u32], out: &mut [u32]) {
        out.copy_from_slice(raw);
    }
}
