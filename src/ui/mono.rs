use crate::{
    ui::popup::Popup,
    visualizer::{Direction, Orientation, Visualizer, mono::MonoVisualizer},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Widget, WidgetRef},
};
use tui_barchart_ext::barchart::{Bar, BarChart};

impl WidgetRef for MonoVisualizer {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        //TODO: maybe only recalculate this on terminal resize?

        match self.direction {
            Direction::Vertical => render_vert(self, area, buf),
            Direction::Horizontal => render_horiz(self, area, buf),
        };

        Popup::render(self, &self.mode, area, buf);
    }
}

fn render_vert(vis: &MonoVisualizer, area: Rect, buf: &mut Buffer) {
    let bars = vis.bars() as u16;
    let width = area.width;
    let height = area.height as u64;

    // bars * (bar_width + 1) = width + 1
    // bar_width + 1 = (width + 1)/bars
    // bar_width = ((width + 1)/bars) - 1
    let bar_width = ((width + 1) / bars) - 1;

    let chart = match vis.orientation {
        Orientation::Inverted => vertical_barchart(vis, bar_width, height).inverted(),
        _ => vertical_barchart(vis, bar_width, height),
    };

    chart.render(area, buf);
}

fn render_horiz(vis: &MonoVisualizer, area: Rect, buf: &mut Buffer) {
    let bars = vis.bars() as u16;
    let width = area.width as u64;
    let height = area.height;

    let bar_width = ((height + 1) / bars) - 1;

    let chart = match vis.orientation {
        Orientation::Inverted => horizontal_barchart(vis, bar_width, width).inverted(),
        _ => horizontal_barchart(vis, bar_width, width),
    };

    chart.render(area, buf);
}

fn horizontal_barchart(vis: &MonoVisualizer, bar_width: u16, width: u64) -> BarChart<'static> {
    let bars = bars(vis);
    BarChart::horizontal(bars)
        .bar_width(bar_width)
        .max(width * 8)
}

fn vertical_barchart(vis: &MonoVisualizer, bar_width: u16, height: u64) -> BarChart<'static> {
    let bars = bars(vis);
    BarChart::vertical(bars)
        .bar_width(bar_width)
        .max(height * 8) // should give the max resolution (each cell has 8 ticks)
}

fn bars(vis: &MonoVisualizer) -> Vec<Bar<'static>> {
    vis.output()
        .iter()
        .map(|amp| bar(*amp, vis.color()))
        .collect()
}

fn bar(amp: u32, color: Color) -> Bar<'static> {
    let blank = String::new();
    Bar::default()
        .value(amp as u64)
        .text_value(blank)
        .style(Style::new().fg(color))
}
