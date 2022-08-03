use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph, Tabs, Widget, Wrap},
};

#[derive(Default)]
pub struct Response<'b> {
    block: Block<'b>,
}

impl Widget for Response<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = ["Content"].iter().cloned().map(Spans::from).collect();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider("|")
            .render(chunks[0], buf);
    }
}
