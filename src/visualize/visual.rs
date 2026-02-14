use ratatui::style::Color;

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
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Default,
    ColorPick,
    ShowStats,
    ShowKeys,
    ShowInput,
}

pub(super) const DEFAULT_COLOR_INDEX: usize = 5;
pub const COLORS: [Color; 8] = [
    Color::White,
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Gray,
];

#[derive(Default)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Default)]
pub enum Orientation {
    Normal,
    #[default]
    Centered,
    Inverted,
}
