use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::ui::popup::HORIZ_PERCENT;

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

pub struct ColorPickPopup {
    pub colors: &'static [Color],
    pub index: usize,
    pub color: Color,
}

impl ColorPickPopup {
    pub fn new(color: Color, colors: &'static [Color], index: usize) -> Self {
        Self {
            colors,
            index,
            color,
        }
    }
}
