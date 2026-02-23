use ratatui::style::Color;

pub mod mono;
pub mod stereo;
pub mod visual;
pub mod waterfall;

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

impl From<Direction> for ratatui::layout::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Vertical => ratatui::layout::Direction::Vertical,
            Direction::Horizontal => ratatui::layout::Direction::Horizontal,
        }
    }
}

#[derive(Default)]
pub enum Orientation {
    #[default]
    Normal,
    Centered,
    Inverted,
}
