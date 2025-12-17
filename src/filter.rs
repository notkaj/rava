use std::cmp;

pub trait Filter {
    fn apply(&self, raw: &[u32], out: &mut [u32]);
}

pub struct NormalFilter {
    rate_of_decay: f32,
}

impl Default for NormalFilter {
    fn default() -> Self {
        // TODO: make this a const or some shit
        Self::new(0.10)
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
            let curr = out[i];
            let decay = (curr as f32 * self.rate_of_decay).ceil() as u32;
            // let new = curr.saturating_sub(decay);
            let new = curr - decay; // this doesn't overflow somehow
            out[i] = cmp::max(new, *e);
        }
    }
}
