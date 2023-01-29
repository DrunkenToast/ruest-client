use std::sync::{Arc, Mutex};

use crossterm::event::KeyEvent;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget},
};

use crate::{
    app::{Action, InputMode, Movement, PaneType},
    component::{
        input_line::{self, InputLine, InputLineState},
        Component,
    },
    keys::NormalKeyAction,
    pane::Pane,
    ui::theme::GlobalTheme,
};

use super::RightStatePane;

#[derive(Debug)]
pub struct RequestState {
    tab_index: usize,
    active: bool,
    theme: GlobalTheme,
    pub hostname_input_state: input_line::InputLineState,
    selected_method: Arc<Mutex<reqwest::Method>>,
}

impl Component for RequestState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        match &mut self.hostname_input_state.input_mode() {
            InputMode::Normal => match NormalKeyAction::from(key_event) {
                NormalKeyAction::PrevTab => {
                    self.prev();
                    None
                }
                NormalKeyAction::NextTab => {
                    self.next();
                    None
                }
                NormalKeyAction::InsertMode => {
                    self.hostname_input_state.set_input_mode(InputMode::Editing);
                    None
                }
                // TODO: Tabs should accept focus, think about how to solve this with the input line.
                //
                // NormalKeyAction::Accept => {
                //     if Request::OPTIONS[self.tab_index] == "Body" {
                //        self.body_input_state.set_input_mode(InputMode::Editing);
                //     }
                //     None
                // }
                key => key.relative_or_none(),
            },
            InputMode::Editing => match self.hostname_input_state.handle_key(key_event) {
                Some(Action::InputResult(input_line::InputResult::Accepted)) => {
                    //TODO: Do stuff here B)
                    None
                }
                Some(Action::InputResult(input_line::InputResult::Canceled)) => None,
                _ => None,
            },
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    #[inline(always)]
    fn input_mode(&self) -> InputMode {
        self.hostname_input_state.input_mode()
    }
}

impl Pane for RequestState {
    fn relative_pane(&self, dir: crate::app::Movement) -> Option<PaneType> {
        match dir {
            Movement::Up => None,
            Movement::Down => None,
            Movement::Left => Some(PaneType::RequestList),
            Movement::Right => Some(PaneType::Right(RightStatePane::Response)),
        }
    }

    fn active_pane(&mut self, _pane: &PaneType) -> &mut dyn Pane {
        debug_assert!(matches!(PaneType::Right(RightStatePane::Request), _pane));
        self
    }
}

impl RequestState {
    const TAB_LEN: usize = Request::OPTIONS.len();
    pub fn new(theme: GlobalTheme, selected_method: Arc<Mutex<reqwest::Method>>) -> Self {
        Self {
            tab_index: 0,
            hostname_input_state: InputLineState::new(
                "".to_string(),
                "Enter a hostname".to_string(),
                theme.clone(),
            ),
            theme,
            active: false,
            selected_method,
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
}

#[derive(Clone, Default)]
pub struct Request {}

impl Request {
    const OPTIONS: &'static [&'static str] = &["Query", "Headers", "Auth", "Body"];
}

impl StatefulWidget for Request {
    type State = RequestState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title("Request")
            .borders(Borders::ALL)
            .style(state.theme.block(state.active));
        let request_area = block.inner(area);
        block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(request_area);

        let titles = Self::OPTIONS.iter().cloned().map(Spans::from).collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(state.tab_index)
            .highlight_style(state.theme.selected());

        let inner = Block::default()
            .title(Self::OPTIONS[state.tab_index])
            .borders(Borders::ALL)
            .style(state.theme.block(state.active()));

        let bar_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[0]);

        let val = match state.selected_method.lock() {
            Ok(method) => method.to_string(),
            Err(_) => String::from("ERROR"),
        };
        let method_block = Paragraph::new(val)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(state.theme.block(false)),
            )
            .style(state.theme.focused())
            .alignment(Alignment::Center);
        let area = bar_chunks[0];
        method_block.render(area, buf);

        let editing = state.hostname_input_state.input_mode() == InputMode::Editing;
        let hostname_block = Block::default()
            .borders(Borders::ALL)
            .style(state.theme.block(editing));
        let area = bar_chunks[1];
        let inner_host_area = hostname_block.inner(area);
        hostname_block.render(area, buf);

        StatefulWidget::render(
            InputLine::default(),
            inner_host_area,
            buf,
            &mut state.hostname_input_state,
        );

        tabs.render(chunks[1], buf);
        inner.render(chunks[2], buf);
    }
}
