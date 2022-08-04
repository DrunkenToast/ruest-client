use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Tabs, Widget},
};

use crate::{app::Actions, keys::KeyAction};

#[derive(Debug, Clone, Default)]
pub struct ResponseState {
    tab_index: usize,
    status_code: reqwest::StatusCode,
}

impl ResponseState {
    const TAB_LEN: usize = Response::OPTIONS.len();
    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % Self::TAB_LEN;
    }
    pub fn prev(&mut self) {
        self.tab_index = self.tab_index.checked_sub(1).unwrap_or(Self::TAB_LEN - 1);
    }
    pub fn select(&mut self, index: usize) {
        assert!(index < Self::TAB_LEN);

        self.tab_index = index;
    }

    pub fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        match key {
            KeyAction::PrevTab => {
                self.prev();
                None
            }
            KeyAction::NextTab => {
                self.next();
                None
            }
            KeyAction::Accept => None,
            key => key.relative_or_none(),
        }
    }
}
#[derive(Default)]
pub struct Response<'b> {
    block: Option<Block<'b>>,
}

impl<'b> Response<'b> {
    const OPTIONS: &'static [&'static str] = &["Content", "Headers", "Cookies"];

    pub fn block(mut self, block: Block<'b>) -> Response<'b> {
        self.block = Some(block);
        self
    }
}

impl<'b> StatefulWidget for Response<'b> {
    type State = ResponseState;
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let request_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        let titles = Self::OPTIONS.iter().cloned().map(Spans::from).collect();
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
        // TODO: Get status code based on respone
        // TODO: Style status code based on range of status
        state.status_code = reqwest::StatusCode::NOT_FOUND;
        Widget::render(
            Table::new([Row::new([Cell::from(Spans::from(vec![
                Span::raw(" Status: "),
                Span::styled(
                    state.status_code.as_str(),
                    Style::default().fg(Color::Green),
                ),
            ]))])])
            .widths(&[Constraint::Length(12)]),
            chunks[0],
            buf,
        );
        Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(state.tab_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider("|")
            .render(chunks[1], buf);
    }
}

impl Widget for Response<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ResponseState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
