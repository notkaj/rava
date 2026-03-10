use crate::{
    ui::popup::Popup,
    visualizer::{
        mono::MonoVisualizer,
        {Direction, Visualizer},
    },
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
        let bars = self.bars() as u16;
        let width = area.width as u64;
        let height = area.height as u64;

        let chart = match self.direction {
            Direction::Vertical => {
                let bar_width = width as u16 / (bars + 1);
                vertical_barchart(self, bar_width, height)
            }
            Direction::Horizontal => {
                let bar_width = height as u16 / (bars + 1);
                horizontal_barchart(self, bar_width, width)
            }
        };

        chart.render(area, buf);
        Popup::render(self, &self.mode, area, buf);
    }
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
