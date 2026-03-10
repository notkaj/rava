pub mod capturer;
pub mod pipewire;

pub const DEFAULT_QUANT: usize = 1024;
pub const DEFAULT_RATE: usize = 44100;

pub(crate) fn max_cap() -> Option<f32> {
    capturer::capture()
        .unwrap_or_default()
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .copied()
}
