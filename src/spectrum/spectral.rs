pub(super) const DEFAULT_RANGE_COUNT: usize = 72;

pub trait Spectral {
    // fn init(&mut self);
    fn update(&mut self);
    fn add_range(&mut self);
    fn remove_range(&mut self);
    fn adjust_scale(&mut self, value: f32);
    fn sample_rate(&self) -> usize;
    fn channels(&self) -> usize;
    fn max(&self) -> Option<u32>;
}

pub(super) const RATIO: f32 = 0.10;
pub(super) const OFFSET: usize = 0;
pub(super) const DEFAULT_SCALE: f32 = 48.0;
