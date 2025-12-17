use ratatui::{
    buffer::Buffer,
    layout::Rect,
    // layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Widget},
};

use crate::app::App;
use crate::filter::NormalFilter;
use crate::visualizer::Visualizer;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Visualizer::default().render(area, buf)
        self.visualizer.render(area, buf);
    }
}

impl Widget for &Visualizer<NormalFilter> {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let block = Block::bordered()
        //     .title("rava")
        //     .title_alignment(Alignment::Center)
        //     .border_type(BorderType::Rounded);

        // let text = format!(
        //     "This is a tui template.\n\
        //         Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        //         Press left and right to increment and decrement the counter respectively.\n\
        //         Counter: {}",
        //     self.counter
        // );

        // let paragraph = Paragraph::new(text)
        //     // .block(block)
        //     .fg(Color::Cyan)
        //     // .bg(Color::Black)
        //     .centered();
        //
        // paragraph.render(area, buf);

        // let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);

        // TODO: maybe only recalculate this on terminal resize?
        let bars = self.bars() as u16;
        let width = area.width;
        let height = area.height as u64;

        let bar_width = width / (bars + 1);

        let chart = vertical_barchart(self, bar_width, height);

        chart.render(area, buf)
    }
}

fn vertical_barchart(
    vis: &Visualizer<NormalFilter>,
    bar_width: u16,
    height: u64,
) -> BarChart<'static> {
    let bars: Vec<Bar> = vis
        .out
        .iter()
        .map(|amp| vertical_bar(*amp, vis.color))
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
