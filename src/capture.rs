pub mod capturer;
pub mod pipewire;

pub const DEFAULT_QUANT: usize = 2048;
pub const DEFAULT_RATE: usize = 48000;

pub(crate) fn max_cap() -> Option<f32> {
    capturer::capture()
        .unwrap_or_default()
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .copied()
}
