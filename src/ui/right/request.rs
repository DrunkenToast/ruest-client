use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};

#[derive(Debug, Clone, Default)]
pub struct RequestState {
    tab_index: usize,
}

impl RequestState {
    const TAB_LEN: usize = Request::OPTIONS.len();

    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % Self::TAB_LEN;
    }

    pub fn prev(&mut self) {
        self.tab_index = self.tab_index.checked_sub(1).unwrap_or(Self::TAB_LEN -1 );
    }

    pub fn select(&mut self, index: usize) {
        assert!(index < Self::TAB_LEN);

        self.tab_index = index;
    }
}

#[derive(Default, Clone)]
pub struct Request<'b> {
    block: Option<Block<'b>>,
}

impl Request<'_> {
    const OPTIONS: &'static [&'static str] = &["Query", "Headers", "Auth", "Body"];
}

impl<'b> StatefulWidget for Request<'b> {
    type State = RequestState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let request_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(request_area);

        let p1 = Paragraph::new("HOSTNAME")
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let titles = Self::OPTIONS
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::Green)),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(state.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );

        let inner = match state.tab_index {
            0 => Block::default().title(Self::OPTIONS[0]).borders(Borders::ALL),
            1 => Block::default().title(Self::OPTIONS[1]).borders(Borders::ALL),
            2 => Block::default().title(Self::OPTIONS[2]).borders(Borders::ALL),
            3 => Block::default().title(Self::OPTIONS[3]).borders(Borders::ALL),
            _ => unreachable!(),
        };

        p1.render(chunks[0], buf);
        tabs.render(chunks[1], buf);
        inner.render(chunks[2], buf);
    }
}

impl Widget for Request<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = RequestState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
