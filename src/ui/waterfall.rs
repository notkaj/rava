use core::f64;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{
        Widget, WidgetRef,
        canvas::{Canvas, Line},
    },
};

use crate::{
    ui::popup,
    visualizer::{Visualizer, waterfall::Waterfall},
};

impl WidgetRef for Waterfall {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let amps = self.out[0].len();

        let data = &self.out;
        let gap = 1.0;
        Canvas::default()
            .x_bounds([0.0, (amps - 1) as f64])
            .y_bounds([area.top() as f64, area.bottom() as f64])
            .paint(|c| {
                for (i, line) in data.iter().enumerate().rev() {
                    let y_offset = i as f64 * gap;
                    let mut j = 0.0;
                    for points in line.windows(2) {
                        let x1 = j;
                        let y1 = points[0] + y_offset;
                        let x2 = j + 1.0;
                        let y2 = points[1] + y_offset;
                        //         let line_gap = 1.0;
                        //         let mut line_y_offset = -line_gap;
                        //
                        //         while y1 + line_y_offset > y2 - 3.0 || y2 + line_y_offset > y1 - 3.0 {
                        //             c.draw(&Line {
                        //                 x1,
                        //                 y1: y1 + line_y_offset,
                        //                 x2,
                        //                 y2: y2 + line_y_offset,
                        //                 color: Color::Black,
                        //             });
                        //             line_y_offset -= line_gap;
                        //         }
                        //
                        //         c.layer();
                        // let color = if i % 3 == 0 {
                        //     Color::Red
                        // } else if i % 3 == 1 {
                        //     Color::Blue
                        // } else {
                        //     Color::Green
                        // };
                        c.draw(&Line {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: Color::Reset,
                        });
                        j += 1.0;
                    }
                    c.layer();
                }
            })
            .render(area, buf);

        popup::Popup::render(self, &self.get_mode(), area, buf);
    }
}
