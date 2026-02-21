use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Clear, Widget},
};

use crate::{spectrum::spectral::max_cap, ui::popup::HORIZ_PERCENT};
use tui_barchart_ext::barchart::{Bar, BarChart};

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
        let vertical = Layout::vertical([Constraint::Length(6)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(HORIZ_PERCENT)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        let b = Block::bordered()
            .title("Input")
            .style(Style::default().fg(self.color))
            .border_type(BorderType::Rounded);

        let inner = b.inner(area);
        let text = Text::from(vec![
            Line::from(format!("Channels: {} | Rate: {}", self.channels, self.rate)),
            Line::from("Maximums:"),
        ]);
        let width = inner.width;
        let chart_max_amp = BarChart::horizontal(vec![
            Bar::default()
                .text_value(format!("{:<3} ", self.max))
                .label("amp")
                .value(self.max as u64),
        ])
        .max((width - 3) as u64 * 8);
        let vert = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);

        let max_cap = (max_cap().unwrap_or_default() * 1000.0) as u64;
        let chart_max_cap = BarChart::horizontal(vec![
            Bar::default()
                .text_value(format!("{:<3} ", max_cap))
                .label("cap")
                .value(max_cap as u64),
        ])
        .max((width - 3) as u64 * 8);

        let [stats, amp, cap] = vert.areas::<3>(inner);

        Clear.render(area, buf); // cool effect if you comment this line out
        b.render(area, buf);
        text.render(stats, buf);
        chart_max_amp.render(amp, buf);
        chart_max_cap.render(cap, buf);
    }
}
