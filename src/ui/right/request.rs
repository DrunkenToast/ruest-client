use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};

use crate::{
    app::{Actions, Movement, PaneType},
    keys::KeyAction,
    pane::Pane,
    ui::theme::GlobalTheme
};

use super::RightStatePane;

#[derive(Debug, Clone)]
pub struct RequestState {
    tab_index: usize,
    active: bool,
    theme: GlobalTheme,
}

impl Pane for RequestState {
    fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        match key {
            KeyAction::PrevTab => {
                self.prev();
                None
            }
            KeyAction::NextTab => {
                self.next();
                None
            }
            key => key.relative_or_none(),
        }
    }

    fn relative_pane(&self, dir: crate::app::Movement) -> Option<PaneType> {
        match dir {
            Movement::Up => None,
            Movement::Down => None,
            Movement::Left => Some(PaneType::RequestList),
            Movement::Right => Some(PaneType::Right(RightStatePane::Response)),
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}

impl RequestState {
    const TAB_LEN: usize = Request::OPTIONS.len();
    pub fn new(theme: GlobalTheme) -> Self {
        Self {
            tab_index: 0,
            theme,
            active: false,
        }
    }

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

#[derive(Clone, Default)]
pub struct Request {}

impl Request {
    const OPTIONS: &'static [&'static str] = &["Query", "Headers", "Auth", "Body"];
}

impl StatefulWidget for Request {
    type State = RequestState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().title("Request").borders(Borders::ALL)
            .style(state.theme.block(state.active));
        let request_area = block.inner(area);
        block.render(area, buf);

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

        let paragraph_hostname = Paragraph::new("HOSTNAME")
            .style(state.theme.hostname())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let titles = Self::OPTIONS.iter().cloned().map(Spans::from).collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(state.tab_index)
            .highlight_style(state.theme.selected());

        paragraph_hostname.render(chunks[0], buf);
        tabs.render(chunks[1], buf);
        if state.tab_index < Self::OPTIONS.len() {
            let inner = Block::default().title(Self::OPTIONS[state.tab_index])
                .borders(Borders::ALL)
                .style(state.theme.block(state.active()));
            inner.render(chunks[2], buf);
        }
        else {
            unreachable!()
        }
    }
}

