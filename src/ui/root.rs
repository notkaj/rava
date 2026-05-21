use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Widget, WidgetRef},
};

use crate::{app::App, visualizer::Visualizer};

impl<T: Visualizer + WidgetRef> Widget for &App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, center, _] = Layout::vertical([
            Constraint::Length(self.top_margin),
            Constraint::Fill(1),
            Constraint::Length(self.bottom_margin),
        ])
        .areas(area);
        let [_, main, _] = Layout::horizontal([
            Constraint::Length(self.left_margin),
            Constraint::Fill(1),
            Constraint::Length(self.right_margin),
        ])
        .areas(center);

        self.visualizer.render_ref(main, buf);
    }
}
