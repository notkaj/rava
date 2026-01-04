use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::filter::Filter;
use crate::popup::{KeysPopup, StatsPopup};
use crate::visualizer::Visualizer;
use crate::{
    app::App,
    popup::{HORIZ_PERCENT, VERT_PERCENT},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Visualizer::default().render(area, buf)
        self.visualizer.render(area, buf);
    }
}

impl<T: Filter> Widget for &Visualizer<T> {
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

        chart.render(area, buf);

        if self.show_stats || self.show_keys {
            let vertical =
                Layout::vertical([Constraint::Percentage(VERT_PERCENT)]).flex(Flex::Center);
            let horizontal =
                Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
            let [area] = vertical.areas(area);
            let [area] = horizontal.areas(area);
            Clear.render(area, buf); // cool effect if you comment this line out
            if self.show_stats {
                let popup = StatsPopup::new(self.sample_rate(), self.sample_len(), self.channels());
                popup.render(area, buf);
            } else {
                let popup = KeysPopup::new();
                popup.render(area, buf);
            };
        }
    }
}

impl Widget for &StatsPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = Text::from(vec![
            Line::from(format!("Sample Rate: {}", self.sample_rate)),
            Line::from(format!("Sample Size: {}", self.sample_len)),
            Line::from(format!("Channels: {}", self.channels)),
        ]);
        let p = Paragraph::new(text)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Stats")
                    .border_type(BorderType::Rounded),
            );
        p.render(area, buf);
    }
}

impl Widget for &KeysPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //TODO: move all this into a const or something
        let text = Text::from(vec![
            Line::from("h -> decrease bars"),
            Line::from("j -> decrease scale"),
            Line::from("l -> increase bars"),
            Line::from("k -> increase scale"),
            Line::from("? -> show keys"),
            Line::from("s -> show stats"),
            Line::from("c -> close popup"),
            Line::from("q -> close app"),
        ]);
        let p = Paragraph::new(text)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Keys")
                    .border_type(BorderType::Rounded),
            );
        p.render(area, buf);
    }
}

fn vertical_barchart<T: Filter>(
    vis: &Visualizer<T>,
    bar_width: u16,
    height: u64,
) -> BarChart<'static> {
    let bars: Vec<Bar> = vis
        .output()
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
