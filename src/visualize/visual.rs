use ratatui::style::Color;

use crate::visualize::Mode;

pub trait Visual {
    fn update(&mut self);
    fn add_bar(&mut self);
    fn remove_bar(&mut self);
    fn increment_scale(&mut self);
    fn decrement_scale(&mut self);
    fn color(&self) -> Color;
    fn next_color(&mut self);
    fn prev_color(&mut self);
    fn sample_rate(&self) -> usize;
    fn channels(&self) -> usize;
    fn sample_len(&self) -> usize;
    fn bars(&self) -> usize;
    fn input_max(&self) -> u32;
    fn color_index(&self) -> usize;
    fn get_mode(&self) -> Mode;
    fn set_mode(&mut self, mode: Mode);
}
