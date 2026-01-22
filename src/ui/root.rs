use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.visualizer.render(area, buf);
    }
}
