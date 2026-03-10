use super::Filter;

const DEFAULT_RATE_OF_DECAY: f32 = 0.05;
const DEFAULT_RATE_OF_INCREASE: f32 = 0.75;

pub struct NormalFilter {
    rate_of_decay: f32,
    rate_of_increase: f32,
}

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
