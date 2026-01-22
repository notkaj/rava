use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Bar, BarChart, Block, BorderType, Clear, Widget},
};

use crate::ui::popup::HORIZ_PERCENT;

pub struct InputPopup {
    pub color: Color,
    pub max: u32,
    pub channels: usize,
    pub rate: usize,
}

impl InputPopup {
    pub fn new(color: Color, max: u32, channels: usize, rate: usize) -> Self {
        Self {
            color,
            max,
            channels,
            rate,
        }
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
