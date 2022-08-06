use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::StatefulWidget,
};

use request::{Request, RequestState};
use response::{Response, ResponseState};

use super::theme::GlobalTheme;

mod request;
mod response;

#[derive(Debug)]
pub struct RightState {
    pub request_state: RequestState,
    pub response_state: ResponseState,
    // pub local_pane: RightStatePane,
}

#[derive(Debug, Copy, Clone)]
pub enum RightStatePane {
    Request,
    Response,
}

#[derive(Default)]
pub struct Right;

impl RightState {
    pub fn new(theme: GlobalTheme) -> Self {
        Self {
            request_state: RequestState::new(theme.clone()),
            response_state: ResponseState::new(theme),
        }
    }
}

impl StatefulWidget for Right {
    type State = RightState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        StatefulWidget::render(Request::default(), chunks[0], buf, &mut state.request_state);
        StatefulWidget::render(
            Response::default(),
            chunks[1],
            buf,
            &mut state.response_state,
        );
    }
}
