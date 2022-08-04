use super::ui::{requests_list::RequestsList, right::RightState};
use crossterm::event::KeyEvent;

use crate::keys::KeyAction;

#[derive(Debug, Default, Clone)]
pub enum Pane {
    #[default]
    RequestList,
    Request,
    Response,
    Collections,
    Relative(RelativePane),
}

#[derive(Debug, Clone)]
pub enum RelativePane {
    Up,
    Down,
    Left,
    Right,
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
        if let Some(pane) = match self.active_pane {
            Pane::RequestList => self.requests_list.handle_key(KeyAction::from(key)),
            Pane::Request => self
                .right_state
                .request_state
                .handle_key(KeyAction::from(key)),
            Pane::Response => self
                .right_state
                .response_state
                .handle_key(KeyAction::from(key)),
            _ => None,
        } {
            match pane {
                Pane::Relative(_) => {}
                pane => self.active_pane = pane,
            }
        }
    }
}
