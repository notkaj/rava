use crate::event::TICK_FPS;

use super::Filter;

const DEFAULT_NOISE_REDUCT: f32 = 0.77;

pub struct CavaFilter {
    curr_peaks: Vec<f32>,
    fall_vals: Vec<f32>,
    noise_reduct: f32,
    prev_amps: Vec<f32>,
    post_int_vals: Vec<f32>,
    gravity_mod: f32,
}

impl CavaFilter {
    fn new(len: usize, noise_reduct: f32) -> Self {
        let curr_peaks = vec![0.0; len];
        let fall_vals = vec![0.0; len];
        let prev_amps = vec![0.0; len];
        let post_int_vals = vec![0.0; len];
        let gravity_mod = ((60.0 / TICK_FPS as f32).powf(2.5) * 1.54 / noise_reduct).max(1.0);
        Self {
            curr_peaks,
            fall_vals,
            noise_reduct,
            prev_amps,
            post_int_vals,
            gravity_mod,
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

        for (i, &amp) in input.iter().enumerate() {
            let prev = self.prev_amps[i];
            let mut res = amp;
            if amp < prev && self.noise_reduct > 0.1 {
                res = self.curr_peaks[i] * (1.0 - (self.fall_vals[i].powi(2) * self.gravity_mod));
                res = res.max(0.0);
                self.fall_vals[i] -= 0.028;
            } else {
                self.curr_peaks[i] = amp;
                self.fall_vals[i] = 0.0;
            }
            self.prev_amps[i] = res;

            // process integral smoothing
            self.post_int_vals[i] = self.post_int_vals[i] * self.noise_reduct + res;
            out[i] = self.post_int_vals[i] as u32;
        }
    }
}
