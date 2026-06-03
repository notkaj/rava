use ratatui::style::Color;

pub mod mono;
pub mod stereo;
pub mod waterfall;
pub mod waveform;

#[derive(Debug, Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Default,
    ColorPick,
    ShowStats,
    ShowKeys,
    ShowInput,
}

pub(super) const DEFAULT_COLOR_INDEX: usize = 5;
pub const COLORS: [Color; 13] = [
    Color::White,
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
    Color::Gray,
    // these are temporary, but i want them here for now
    Color::from_u32(0xb4befe), // Lavender
    Color::from_u32(0xf5c2e7), // Pink
    Color::from_u32(0xcba6f7), //Mauve
    Color::from_u32(0x94e2d5), //Teal
];

#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

impl From<Direction> for ratatui::layout::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Vertical => ratatui::layout::Direction::Vertical,
            Direction::Horizontal => ratatui::layout::Direction::Horizontal,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Orientation {
    #[default]
    Normal,
    Centered,
    Inverted,
}

pub trait Visualizer {
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
