use crate::{
    filter::Filter,
    ui::popup::{colors::ColorPickPopup, input::InputPopup, keys::KeysPopup, stats::StatsPopup},
    visualize::mono::MonoVisualizer,
    visualize::visual::{COLORS, Mode, Visual},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Widget},
};

impl<T: Filter> Widget for &MonoVisualizer<T> {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO: maybe only recalculate this on terminal resize?
        let bars = self.bars() as u16;
        let width = area.width;
        let height = area.height as u64;

        let bar_width = width / (bars + 1);
        let color = self.color();

        let chart = vertical_barchart(self, color, bar_width, height);

        chart.render(area, buf);

        match self.mode {
            Mode::Default => (),
            Mode::ShowStats => {
                let popup = StatsPopup::new(
                    color,
                    self.sample_rate(),
                    self.sample_len(),
                    self.channels(),
                );
                popup.render(area, buf);
            }
            Mode::ShowKeys => {
                let popup = KeysPopup::new(color);
                popup.render(area, buf);
            }
            Mode::ColorPick => {
                let popup = ColorPickPopup::new(color, COLORS.as_slice(), self.color_index);
                popup.render(area, buf);
            }
            Mode::ShowInput => {
                let popup =
                    InputPopup::new(color, self.input_max(), self.channels(), self.sample_rate());
                popup.render(area, buf);
            }
        }
    }
}

fn vertical_barchart<T: Filter>(
    vis: &MonoVisualizer<T>,
    color: Color,
    bar_width: u16,
    height: u64,
) -> BarChart<'static> {
    let bars: Vec<Bar> = vis
        .output()
        .iter()
        .map(|amp| vertical_bar(*amp, color))
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
