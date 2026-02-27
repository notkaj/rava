use std::f32::consts::PI;

pub mod average;
pub mod spectral;
pub mod stereo;

pub(super) const DEFAULT_RANGE_COUNT: usize = 36;
pub(super) const RATIO: f32 = 0.06;
pub(super) const OFFSET: usize = 0;
pub(super) const DEFAULT_SCALE: f32 = 24.0;

#[allow(dead_code)]
pub(crate) fn hann_multipliers(size: usize) -> Vec<f32> {
    let mut res = vec![0.0; size];
    for (i, mul) in res.iter_mut().enumerate() {
        *mul = 0.5 * (1.0 - (2.0 * PI * i as f32 / (size - 1) as f32).cos());
    }
    res
}

// executes in place
pub(crate) fn apply_hann_window(spectrum: &mut [f32], mul: &[f32]) {
    for (i, amp) in spectrum.iter_mut().enumerate() {
        *amp *= mul[i]
    }
}
