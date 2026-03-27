use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, WidgetRef},
};

use crate::{app::App, visualizer::Visualizer, warn};

impl<T: Visualizer + WidgetRef> Widget for &App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.visualizer.render_ref(area, buf);

        if !warn::has_warnings() {
            return;
        }

        let warnings = Warnings {
            warnings: warn::warnings(),
        };

        warnings.render(area, buf);
    }
}

impl Widget for Warnings {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let vertical = Layout::vertical([Constraint::Length(10)]).flex(Flex::Start);
        let horizontal = Layout::horizontal([Constraint::Percentage(30)]).flex(Flex::End);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        let warnings = self.warnings;
        let text = Text::from(
            warnings
                .iter()
                .map(|&t| Line::from(t))
                .collect::<Vec<Line>>(),
        );
        let p = Paragraph::new(text)
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Warnings")
                    .border_type(BorderType::Rounded),
            );
        Clear.render(area, buf); // cool effect if you comment this line out
        p.render(area, buf);
    }
}

struct Warnings {
    warnings: Vec<&'static str>,
}
