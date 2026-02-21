use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, WidgetRef},
};

use crate::{app::App, visualize::visual::Visual};

impl<T: Visual + WidgetRef> Widget for &App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.visualizer.render_ref(area, buf);
    }
}
