use core::f64;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Widget, WidgetRef},
};

use crate::{
    ui::popup,
    visualize::{visual::Visual, waterfall::Waterfall},
};

impl WidgetRef for Waterfall {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let width = area.width;
        let height = area.height;
        let data = data(self, width, height);
        let datasets = Dataset::default()
            .data(&data)
            .graph_type(GraphType::Line)
            .marker(symbols::Marker::Braille);
        let chart = Chart::new(vec![datasets])
            .x_axis(Axis::default().bounds([0.0, self.out[0].len() as f64 - 1.0]))
            .y_axis(Axis::default().bounds([0.0, 200.0]));
        chart.render(area, buf);

        popup::Popup::render(self, &self.get_mode(), area, buf);
    }
}

fn data(waterfall: &Waterfall, width: u16, height: u16) -> Vec<(f64, f64)> {
    let mut res = vec![(0.0, 0.0); waterfall.out[0].len()];
    let mut x = 0.0;
    for (i, datum) in waterfall.out[0].iter().enumerate() {
        res[i] = (x, *datum);
        x += 1.0;
    }
    res
    // let mut rng = rand::rng();
    // let mut data = Vec::new();
    // for i in 0..200 {
    //     data.push((i as f64, rng.random::<f64>() * 20.0));
    // }
    // data
}
