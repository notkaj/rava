use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, BorderType, Borders, Clear, List, ListState, Paragraph,
        StatefulWidget, Widget,
    },
};

use crate::{app::App, popup::HORIZ_PERCENT};
use crate::{filter::Filter, visualizer::Mode};
use crate::{popup::InputPopup, visualizer::Visualizer};
use crate::{
    popup::{ColorPickPopup, KeysPopup, StatsPopup},
    visualizer::COLORS,
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

impl Widget for &StatsPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(5)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        let text = Text::from(vec![
            Line::from(format!("Sample Rate: {}", self.sample_rate)),
            Line::from(format!("Sample Size: {}", self.sample_len)),
            Line::from(format!("Channels: {}", self.channels)),
        ]);
        let p = Paragraph::new(text)
            .style(Style::default().fg(self.color))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Stats")
                    .border_type(BorderType::Rounded),
            );
        Clear.render(area, buf); // cool effect if you comment this line out
        p.render(area, buf);
    }
}

impl Widget for &KeysPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(11)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        //TODO: move all this into a const or something
        let text = Text::from(vec![
            Line::from("h -> decrease bars"),
            Line::from("j -> decrease scale"),
            Line::from("k -> increase scale"),
            Line::from("l -> increase bars"),
            Line::from("? -> show keys"),
            Line::from("s -> show stats"),
            Line::from("c -> show colors"),
            Line::from("q -> close app"),
            Line::from("ESC -> close popup"),
        ]);
        let p = Paragraph::new(text)
            .style(Style::default().fg(self.color))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Keys")
                    .border_type(BorderType::Rounded),
            );
        Clear.render(area, buf);
        p.render(area, buf);
    }
}

impl Widget for &ColorPickPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(10)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        let list = List::new(self.colors.iter().map(|c| c.to_string()))
            .style(Style::default().fg(self.color))
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Colors")
                    .border_type(BorderType::Rounded),
            );

        let index = self.index;
        let mut list_state = ListState::default().with_selected(Some(index));

        Clear.render(area, buf);
        StatefulWidget::render(list, area, buf, &mut list_state);
    }
}

impl Widget for &InputPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(4)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        let b = Block::bordered()
            .title("Input")
            .style(Style::default().fg(self.color))
            .border_type(BorderType::Rounded);

        let inner = b.inner(area);
        let text = Text::from(vec![Line::from(format!(
            "Channels: {} | Rate: {}",
            self.channels, self.rate
        ))]);
        let width = inner.width;
        let chart = BarChart::horizontal(vec![
            Bar::default()
                .label(format!("{:<3} ", self.max))
                .text_value("")
                .value(self.max as u64),
        ])
        .max((width - 3) as u64 * 8);
        let vert = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [stats, max] = vert.areas::<2>(inner);

        Clear.render(area, buf); // cool effect if you comment this line out
        b.render(area, buf);
        text.render(stats, buf);
        chart.render(max, buf);
    }
}

fn vertical_barchart<T: Filter>(
    vis: &Visualizer<T>,
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
