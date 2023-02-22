use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crossterm::event::KeyEvent;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget},
};
use tui_textarea::TextArea;

use crate::{
    app::{Action, InputMode, Movement, PaneType},
    component::Component,
    keys::NormalKeyAction,
    pane::Pane,
    ui::theme::GlobalTheme,
};

use super::RightStatePane;

pub struct RequestState<'a> {
    tab_index: usize,
    active: bool,
    theme: GlobalTheme,
    pub input_line: TextArea<'a>,
    pub body: TextArea<'a>,
    input_mode: InputMode,
    selected_method: Arc<Mutex<reqwest::Method>>,
}

impl<'a> Component for RequestState<'a> {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        match &mut self.input_mode {
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
                    self.input_mode = InputMode::Hostname;
                    None
                }
                // TODO: Tabs should accept focus, think about how to solve this with the input line.
                NormalKeyAction::Accept => {
                    if Request::OPTIONS[self.tab_index] == "Body" {
                        self.input_mode = InputMode::Body;
                    }
                    None
                }
                key => key.relative_or_none(),
            },

            InputMode::Hostname => match NormalKeyAction::from(key_event) {
                NormalKeyAction::Exit | NormalKeyAction::Accept => {
                    self.input_mode = InputMode::Normal;
                    None
                }
                _ => {
                    self.input_line.input(key_event);
                    None
                }
            },
            InputMode::Body => match NormalKeyAction::from(key_event) {
                NormalKeyAction::Exit => {
                    self.input_mode = InputMode::Normal;
                    None
                }
                _ => {
                    self.body.input(key_event);
                    None
                }
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
        self.input_mode
    }
}

impl<'a> Pane for RequestState<'a> {
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

impl<'a> RequestState<'a> {
    const TAB_LEN: usize = Request::OPTIONS.len();
    pub fn new(theme: GlobalTheme, selected_method: Arc<Mutex<reqwest::Method>>) -> Self {
        Self {
            tab_index: 0,
            theme,
            active: false,
            input_line: TextArea::default(),
            body: TextArea::from("{\n\n}".lines()),
            input_mode: InputMode::Normal,
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
pub struct Request<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> Request<'a> {
    const OPTIONS: &'static [&'static str] = &["Query", "Headers", "Auth", "Body"];
}

impl<'a> StatefulWidget for Request<'a> {
    type State = RequestState<'a>;
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
            .style(state.theme.block(state.input_mode == InputMode::Body));

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

        let hostname_block = Block::default()
            .borders(Borders::ALL)
            .style(state.theme.block(state.input_mode == InputMode::Hostname));
        let area = bar_chunks[1];
        let inner_host_area = hostname_block.inner(area);
        hostname_block.render(area, buf);
        Widget::render(state.input_line.widget(), inner_host_area, buf);

        if Request::OPTIONS[state.tab_index] == "Body" {
            let area = chunks[2];
            Widget::render(state.body.widget(), inner.inner(area), buf);
        }

        tabs.render(chunks[1], buf);
        inner.render(chunks[2], buf);
    }
}
