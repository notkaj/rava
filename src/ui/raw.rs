use ratatui::widgets::{
    Widget, WidgetRef,
    canvas::{Canvas, Line},
};

use crate::{
    ui::popup::Popup,
    visualizer::{Visualizer, raw::RawMonoVisualizer},
};

impl WidgetRef for RawMonoVisualizer {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let amps = self.output();

        let canvas = Canvas::default()
            .x_bounds([0., (amps.len() - 1) as f64])
            .y_bounds([-20., 20.])
            .paint(|c| {
                let mut j = 0.;
                for points in amps.windows(2) {
                    c.draw(&Line {
                        x1: j,
                        y1: points[0],
                        x2: j + 1.,
                        y2: points[1],
                        color: self.color(),
                    });
                    j += 1.;
                }
            });

        canvas.render(area, buf);

        Popup::render(self, &self.mode, area, buf);
    }
}
