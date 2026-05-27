use ratatui::{
    style::Color,
    widgets::{
        Widget, WidgetRef,
        canvas::{Canvas, Line},
    },
};

use crate::visualizer::raw::RawMonoVisualizer;

impl WidgetRef for RawMonoVisualizer {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let amps = self.output();
        let mid = (area.bottom() as f64 - area.top() as f64) / 2.;

        let canvas = Canvas::default()
            .x_bounds([0., (amps.len() - 1) as f64])
            .y_bounds([f64::from(area.top()), f64::from(area.bottom())])
            .paint(|c| {
                let mut j = 0.;
                for points in amps.windows(2) {
                    c.draw(&Line {
                        x1: j,
                        y1: mid + points[0],
                        x2: j + 1.,
                        y2: mid + points[1],
                        color: Color::Reset,
                    });
                    j += 1.;
                }
            });

        canvas.render(area, buf);
    }
}
