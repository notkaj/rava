use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::ui::popup::HORIZ_PERCENT;
pub struct StatsPopup {
    pub color: Color,
    pub sample_rate: usize,
    pub sample_len: usize,
    pub channels: usize,
}

impl StatsPopup {
    pub fn new(color: Color, sample_rate: usize, sample_len: usize, channels: usize) -> Self {
        Self {
            color,
            sample_rate,
            sample_len,
            channels,
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
