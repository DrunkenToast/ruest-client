use std::str::MatchIndices;

use super::ui::{requests_list::RequestsList, right::RightState};
use crossterm::event::KeyEvent;

use crate::keys::KeyAction;

#[derive(Debug, Default, Clone)]
pub enum Pane {
    #[default]
    RequestList,
    Right,
    Collections,
}

#[derive(Debug, Clone)]
pub enum RelativePane {
    Up,
    Down,
    Left,
    Right,
}
pub enum Actions {
    MoveRelative(RelativePane),
    MoveAbsolute(Pane),
}

#[derive(Debug)]
pub struct App<'r> {
    pub requests_list: RequestsList<&'r str>,
    pub right_state: RightState,
    pub active_pane: Pane,
}

impl<'r> App<'r> {
    pub fn new() -> App<'r> {
        App {
            requests_list: RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]),
            right_state: RightState::default(),
            active_pane: Pane::RequestList,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if let Some(action) = match self.active_pane {
            Pane::RequestList => self.requests_list.handle_key(KeyAction::from(key)),
            Pane::Right => self.right_state.handle_key(KeyAction::from(key)),
            _ => None,
        } {
            if let Actions::MoveRelative(pane) = action {
            } else if let Actions::MoveAbsolute(pane) = action {
                self.active_pane = pane;
            }
        }
    }
}
