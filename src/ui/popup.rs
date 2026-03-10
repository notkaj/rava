use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    ui::popup::{colors::ColorPickPopup, input::InputPopup, keys::KeysPopup, stats::StatsPopup},
    visualizer::{COLORS, Mode, Visualizer},
};

pub(super) mod colors;
pub(super) mod input;
pub(super) mod keys;
pub(super) mod stats;

// pub const VERT_PERCENT: u16 = 30;
pub const HORIZ_PERCENT: u16 = 20;

pub struct Popup;

impl Popup {
    pub fn render<T: Visualizer>(vis: &T, mode: &Mode, area: Rect, buf: &mut Buffer) {
        let color = vis.color();
        match mode {
            Mode::Default => (),
            Mode::ShowStats => {
                let popup = StatsPopup::new(
                    vis.color(),
                    vis.sample_rate(),
                    vis.sample_len(),
                    vis.channels(),
                );
                popup.render(area, buf);
            }
            Mode::ShowKeys => {
                let popup = KeysPopup::new(color);
                popup.render(area, buf);
            }
            Mode::ColorPick => {
                let popup = ColorPickPopup::new(color, COLORS.as_slice(), vis.color_index());
                popup.render(area, buf);
            }
            Mode::ShowInput => {
                let popup =
                    InputPopup::new(color, vis.input_max(), vis.channels(), vis.sample_rate());
                popup.render(area, buf);
            }
        }
    }
}
