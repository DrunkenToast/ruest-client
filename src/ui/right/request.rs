use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget, Wrap},
};

use crate::{
    app::{Actions, InputMode, Movement, PaneType},
    keys::NormalKeyAction,
    pane::Pane,
    ui::theme::GlobalTheme,
};

use super::RightStatePane;

#[derive(Debug, Clone)]
pub struct RequestState {
    tab_index: usize,
    active: bool,
    theme: GlobalTheme,
    input_mode: InputMode,
    // NOTE: maybe replace with Vec<Char>
    hostname: String,
}

impl Pane for RequestState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Actions> {
        match self.input_mode {
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
                    self.input_mode = InputMode::Editing;
                    None
                }
                key => key.relative_or_none(),
            },
            InputMode::Editing => match key_event.code {
                KeyCode::Enter => {
                    self.input_mode = InputMode::Normal;
                    None
                }
                KeyCode::Backspace => {
                    // TODO: maybe a beep sound or flast when this erorrs
                    _ = self.hostname.pop().is_some();
                    None
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    None
                }
                KeyCode::Char(char) => {
                    self.hostname.push(char);
                    None
                }
                _ => None,
            },
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

    fn active_pane(&mut self, _pane: &PaneType) -> &mut dyn Pane {
        debug_assert!(matches!(PaneType::Right(RightStatePane::Request), _pane));

        self
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

impl RequestState {
    const TAB_LEN: usize = Request::OPTIONS.len();
    pub fn new(theme: GlobalTheme) -> Self {
        Self {
            tab_index: 0,
            theme,
            active: false,
            input_mode: InputMode::Normal,
            hostname: String::from("localhost"),
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
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(request_area);

        let paragraph_hostname = Paragraph::new(state.hostname.as_str())
            .style(state.theme.hostname())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let titles = Self::OPTIONS.iter().cloned().map(Spans::from).collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(state.tab_index)
            .highlight_style(state.theme.selected());

        let inner = Block::default()
            .title(Self::OPTIONS[state.tab_index])
            .borders(Borders::ALL)
            .style(state.theme.block(state.active()));

        paragraph_hostname.render(chunks[0], buf);
        tabs.render(chunks[1], buf);
        inner.render(chunks[2], buf);
    }
}
