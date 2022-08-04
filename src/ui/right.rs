use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use request::{Request, RequestState};
use response::{Response, ResponseState};

use crate::{
    app::{Actions, Pane, RelativePane},
    keys::KeyAction,
};

mod request;
mod response;

#[derive(Debug, Default)]
pub struct RightState {
    pub request_state: RequestState,
    pub response_state: ResponseState,
    pub local_pane: LocalPane,
}

impl RightState {
    pub fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        match key {
            KeyAction::MoveLeft => match self.local_pane {
                LocalPane::Response => {
                    self.local_pane = LocalPane::Request;
                    Some(Actions::MoveRelative(RelativePane::Left))
                }
                LocalPane::Request => {
                    self.local_pane = LocalPane::None;
                    Some(Actions::MoveAbsolute(Pane::RequestList))
                }
                LocalPane::None => None,
            },
            KeyAction::MoveRight => match self.local_pane {
                LocalPane::Response => {
                    self.local_pane = LocalPane::None;
                    Some(Actions::MoveAbsolute(Pane::RequestList))
                }
                LocalPane::Request => {
                    self.local_pane = LocalPane::Response;
                    Some(Actions::MoveRelative(RelativePane::Right))
                }
                LocalPane::None => None,
            },
            key => match self.local_pane {
                LocalPane::Request => self.request_state.handle_key(key),
                LocalPane::Response => self.response_state.handle_key(key),
                LocalPane::None => None,
            },
        }
    }
}

#[derive(Debug, Default)]
pub enum LocalPane {
    Request,
    Response,
    #[default]
    None,
}

#[derive(Default)]
pub struct Right;

impl StatefulWidget for Right {
    type State = RightState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        let request_block = Block::default().title("Request").borders(Borders::ALL);
        let response_block = Block::default().title("Response").borders(Borders::ALL);

        StatefulWidget::render(
            Request::default().block(request_block),
            chunks[0],
            buf,
            &mut state.request_state,
        );
        StatefulWidget::render(
            Response::default().block(response_block),
            chunks[1],
            buf,
            &mut state.response_state,
        );
    }
}

impl Widget for Right {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = RightState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
