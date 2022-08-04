use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use request::{Request, RequestState};
use response::{Response, ResponseState};

mod request;
mod response;

#[derive(Debug, Default)]
pub struct RightState {
    pub request_state: RequestState,
    pub response_state: ResponseState,
}

#[derive(Default)]
pub struct Right;

impl StatefulWidget for Right {
    type State = RightState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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
