use std::sync::{Arc, Mutex};

use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::StatefulWidget,
};

use crate::{app::PaneType, component::Component, pane::Pane};

use super::theme::GlobalTheme;

use request::{Request, RequestState};
use response::{Response, ResponseState};

mod request;
mod response;

pub struct RightState {
    pub active: bool,
    pub request_state: RequestState,
    pub response_state: ResponseState,
    // pub local_pane: RightStatePane,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum RightStatePane {
    #[default]
    Request,
    Response,
}

#[derive(Default)]
pub struct Right;

impl RightState {
    pub fn new(theme: GlobalTheme, selected_method: Arc<Mutex<reqwest::Method>>) -> Self {
        Self {
            request_state: RequestState::new(theme.clone(), selected_method),
            response_state: ResponseState::new(theme),
            active: false,
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

impl Component for RightState {
    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Pane for RightState {
    fn active_pane(&mut self, pane: &crate::app::PaneType) -> &mut dyn Pane {
        if let PaneType::Right(pane) = pane {
            match pane {
                RightStatePane::Request => &mut self.request_state,
                RightStatePane::Response => &mut self.response_state,
            }
        } else {
            unreachable!();
        }
    }
}
