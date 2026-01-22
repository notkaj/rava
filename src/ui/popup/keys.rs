use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::ui::popup::HORIZ_PERCENT;

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

pub struct KeysPopup {
    pub color: Color,
}

impl KeysPopup {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
