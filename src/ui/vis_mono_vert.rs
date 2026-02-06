use crate::{
    filter::Filter,
    ui::popup::Popup,
    visualize::{mono::MonoVisualizer, visual::Visual},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};
use tui_barchart_ext::barchart::{Bar, BarChart, BarGroup};

impl<T: Filter> Widget for &MonoVisualizer<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //TODO: maybe only recalculate this on terminal resize?
        let bars = self.bars() as u16;
        let width = area.width;
        let height = area.height as u64;

        let bar_width = width / (bars + 1);

        let chart = vertical_barchart(self, bar_width, height);

        chart.render(area, buf);

        Popup::render(self, &self.mode, area, buf);
    }
}

fn vertical_barchart<T: Filter>(
    vis: &MonoVisualizer<T>,
    bar_width: u16,
    height: u64,
) -> BarChart<'static> {
    let bars: Vec<Bar> = vis
        .output()
        .iter()
        .map(|amp| vertical_bar(*amp, vis.color()))
        .collect();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(bar_width)
        .max(height * 8) // should give the max resolution (each cell has 8 ticks)
}

fn vertical_bar(amp: u32, color: Color) -> Bar<'static> {
    let blank = String::new();
    Bar::default()
        .value(amp as u64)
        .text_value(blank)
        .style(Style::new().fg(color))
}
