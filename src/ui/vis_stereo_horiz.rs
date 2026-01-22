use crate::{
    filter::Filter,
    ui::popup::{colors::ColorPickPopup, input::InputPopup, keys::KeysPopup, stats::StatsPopup},
    visualize::mono::MonoVisualizer,
    visualize::visual::{COLORS, Mode, Visual},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Widget},
};
