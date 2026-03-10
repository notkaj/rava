use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, WidgetRef},
};

use crate::{app::App, visualizer::Visualizer};

impl<T: Visualizer + WidgetRef> Widget for &App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.visualizer.render_ref(area, buf);
    }
}
