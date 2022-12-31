use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::KeyEvent;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, Tabs, Widget, Wrap},
};

use crate::{
    app::{Action, Movement, PaneType},
    component::Component,
    keys::NormalKeyAction,
    pane::Pane,
    ui::theme::GlobalTheme,
};

use super::RightStatePane;

#[derive(Debug, Clone)]
pub struct ResponseState {
    tab_index: usize,
    pub status_code: reqwest::StatusCode,
    theme: GlobalTheme,
    active: bool,
    pub response: String,
    scroll: u16,
}

impl Component for ResponseState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        match NormalKeyAction::from(key_event) {
            NormalKeyAction::PrevTab => {
                self.prev();
                None
            }
            NormalKeyAction::NextTab => {
                self.next();
                None
            }
            NormalKeyAction::MoveUp => {
                self.scroll += 1;
                None
            }
            NormalKeyAction::MoveDown => {
                if self.scroll != 0 {
                    self.scroll -= 1;
                }
                None
            }
            NormalKeyAction::Copy => {
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(self.response.clone()).unwrap();
                None
            }
            key => key.relative_or_none(),
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}

impl Pane for ResponseState {
    fn relative_pane(&self, dir: crate::app::Movement) -> Option<PaneType> {
        match dir {
            Movement::Up => None,
            Movement::Down => None,
            Movement::Left => Some(PaneType::Right(RightStatePane::Request)),
            Movement::Right => Some(PaneType::RequestList),
        }
    }

    fn active_pane(&mut self, _pane: &PaneType) -> &mut dyn Pane {
        debug_assert!(matches!(PaneType::Right(RightStatePane::Response), _pane));

        self
    }
}

impl ResponseState {
    const TAB_LEN: usize = Response::OPTIONS.len();

    pub fn new(theme: GlobalTheme) -> Self {
        Self {
            tab_index: 0,
            status_code: reqwest::StatusCode::default(),
            theme,
            active: false,
            response: String::default(),
            scroll: 0,
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

    pub fn handle_key(&mut self, key: NormalKeyAction) -> Option<Action> {
        match key {
            NormalKeyAction::PrevTab => {
                self.prev();
                None
            }
            NormalKeyAction::NextTab => {
                self.next();
                None
            }
            NormalKeyAction::Accept => None,

            key => key.relative_or_none(),
        }
    }
}

#[derive(Default)]
pub struct Response;

impl Response {
    const OPTIONS: &'static [&'static str] = &["Content", "Headers", "Cookies"];
}

impl StatefulWidget for Response {
    type State = ResponseState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title("Response")
            .borders(Borders::ALL)
            .style(state.theme.block(state.active));
        let request_area = block.inner(area);
        block.render(area, buf);

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

        let response_text = Paragraph::new(state.response.clone())
            .style(state.theme.block(state.active()))
            .scroll((state.scroll, 0))
            .wrap(Wrap { trim: false });
        Widget::render(
            Table::new([Row::new([Cell::from(Spans::from(vec![
                Span::raw(" Status: "),
                Span::styled(
                    state.status_code.as_str(),
                    state.theme.status_code(state.status_code.as_u16()),
                ),
            ]))])])
            .widths(&[Constraint::Length(12)]),
            chunks[0],
            buf,
        );
        Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(state.tab_index)
            .highlight_style(state.theme.selected())
            .divider("|")
            .render(chunks[1], buf);
        response_text.render(chunks[2], buf);
    }
}
