pub mod average;
pub mod spectral;
pub mod stereo;

pub(super) const DEFAULT_RANGE_COUNT: usize = 36;
pub(super) const RATIO: f32 = 0.06;
pub(super) const OFFSET: usize = 0;
pub(super) const DEFAULT_SCALE: f32 = 24.0;

use crate::capture::capturer::capture;
pub(crate) fn max_cap() -> Option<f32> {
    capture()
        .unwrap_or_default()
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .copied()
}
