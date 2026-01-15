pub trait Spectrum {
    fn init(&mut self);
    fn update(&mut self);
    fn sample() -> Vec<f32>;
    fn add_range(&mut self);
    fn remove_range(&mut self);
    fn adjust_scale(&mut self, value: f32);
    fn sample_rate(&self) -> usize;
    fn channels(&self) -> usize;
    fn max(&self) -> Option<u32>;
}
