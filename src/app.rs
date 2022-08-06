use std::rc::Rc;

use super::ui::{requests_list::RequestsList, right::RightState};
use crossterm::event::KeyEvent;

use crate::{keys::KeyAction, pane::Pane, ui::right::RightStatePane};

#[derive(Debug, Default, Clone)]
pub enum PaneType {
    #[default]
    RequestList,
    Right(RightStatePane),
}

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}
pub enum Actions {
    MoveRelative(Movement),
    InputMode,
}

pub struct App<'a> {
    pub requests_list: RequestsList<&'a str>,
    pub right_state: RightState,
    active_pane_type: PaneType,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let requests_list = RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]);
        let right_state = RightState::default();
        App {
            requests_list,
            right_state,
            active_pane_type: PaneType::RequestList,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if let Some(action) = self.active_pane().handle_key(KeyAction::from(key)) {
            if let Actions::MoveRelative(dir) = action {
                dbg!(&dir);
                self.active_pane().set_active(false);
                if let Some(pane) = self.active_pane().relative_pane(dir) {
                    self.active_pane_type = pane;
                    dbg!(&self.active_pane_type);
                }
                self.active_pane().set_active(true);
            }
        }
    }

    pub fn active_pane(&mut self) -> &mut dyn Pane {
        match self.active_pane_type {
            PaneType::RequestList => &mut self.requests_list,
            PaneType::Right(pane) => match pane {
                RightStatePane::Request => &mut self.right_state.request_state,
                RightStatePane::Response => &mut self.right_state.response_state,
            },
        }
    }
}
