use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

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

pub struct RightState<'a> {
    pub active: bool,
    pub request_state: RequestState<'a>,
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
pub struct Right<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> RightState<'a> {
    pub fn new(theme: GlobalTheme, selected_method: Arc<Mutex<reqwest::Method>>) -> Self {
        Self {
            request_state: RequestState::new(theme.clone(), selected_method),
            response_state: ResponseState::new(theme),
            active: false,
        }
    }
}

impl<'a> StatefulWidget for Right<'a> {
    type State = RightState<'a>;

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

impl<'a> Component for RightState<'a> {
    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl<'a> Pane for RightState<'a> {
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
