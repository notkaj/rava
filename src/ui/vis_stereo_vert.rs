use crate::{
    filter::Filter,
    ui::popup::Popup,
    visualize::{
        stereo::{Presentation, StereoVisualizer},
        visual::Visual,
    },
};
use ratatui::{
    buffer::Buffer,
    layout::{/*Constraint, Flex, Layout,*/ Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Widget,
};
use tui_barchart_ext::barchart::{Bar, BarChart, BarGroup};

impl<T: Filter> Widget for &StereoVisualizer<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.presentation {
            Presentation::Horizontal => render_vert(self, area, buf),
            Presentation::Vertical => (),
        }

        Popup::render(self, &self.mode, area, buf);
    }
}

fn render_vert<T: Filter>(vis: &StereoVisualizer<T>, area: Rect, buf: &mut Buffer) {
    let bars = vis.bars() as u16;
    let width = area.width;
    let height = area.height as u64;

    let bar_width = width / (bars * 2) - 1;
    let rem = width - ((bar_width + 1) * bars * 2);

    let [_, main] =
        Layout::horizontal([Constraint::Length(rem / 2 + 1), Constraint::Fill(1)]).areas(area);

    let chart = vertical_barchart(vis, bar_width, height);
    chart.render(main, buf)

    // let (left_chart, right_chart) = vertical_barcharts(vis, bar_width, height);
    //
    // let layout = Layout::horizontal([
    //     Constraint::Fill(1),
    //     // Constraint::Length(1),
    //     Constraint::Fill(1),
    // ]);
    // let [left_area, right_area] = layout.areas(area);
    //
    // left_chart.render(left_area, buf);
    // // Clear.render(gap, buf);
    // right_chart.render(right_area, buf);
}

#[allow(dead_code)]
fn vertical_barchart<T: Filter>(
    vis: &StereoVisualizer<T>,
    bar_width: u16,
    height: u64,
) -> BarChart<'static> {
    let color = vis.color();
    let chan_len = vis.left_out.len();
    let mut bars = Vec::with_capacity(chan_len * 2);
    vis.left_out
        .iter()
        .for_each(|e| bars.push(vertical_bar(*e, color)));
    vis.right_out
        .iter()
        .rev()
        .for_each(|e| bars.push(vertical_bar(*e, color)));
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(bar_width)
        .max(height * 8)
}

#[allow(dead_code)]
fn vertical_barcharts<T: Filter>(
    vis: &StereoVisualizer<T>,
    bar_width: u16,
    height: u64,
) -> (BarChart<'static>, BarChart<'static>) {
    let left_bars: Vec<Bar> = vis
        .left_out
        .iter()
        .map(|amp| vertical_bar(*amp, vis.color()))
        .collect();
    let right_bars: Vec<Bar> = vis
        .right_out
        .iter()
        .rev()
        .map(|amp| vertical_bar(*amp, vis.color()))
        .collect();
    let left = BarChart::default()
        .data(BarGroup::default().bars(&left_bars))
        .bar_width(bar_width)
        .max(height * 8);
    let right = BarChart::default()
        .data(BarGroup::default().bars(&right_bars))
        .bar_width(bar_width)
        .max(height * 8);
    (left, right)
}

fn vertical_bar(amp: u32, color: Color) -> Bar<'static> {
    let blank = String::new();
    Bar::default()
        .value(amp as u64)
        .text_value(blank)
        .style(Style::new().fg(color))
}
