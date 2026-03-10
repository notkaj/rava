use super::Filter;

const DEFAULT_DIFF_RATE: f32 = 0.50;

pub struct SmoothFilter {
    rate_of_diff: f32,
}

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
