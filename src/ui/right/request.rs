use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Rect, Alignment, Layout},
    widgets::{Block, Borders, Widget, Paragraph, Wrap}, style::{Style, Color},
};

#[derive(Default)]
pub struct Request<'b> {
    block: Block<'b>,
}

impl Widget for Request<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let h = area.height;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(h - 4),
                ]
                .as_ref(),
            )
            .split(area);

        let p1 = Paragraph::new("HOSTNAME")
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let p2 = Paragraph::new("TABS")
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let p3 = Paragraph::new("Content")
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        p1.render(chunks[0], buf);
        p2.render(chunks[1], buf);
        p3.render(chunks[2], buf);
    }
}
