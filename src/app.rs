use crossterm::event::KeyEvent;

use crate::{
    keys::KeyAction,
    ui::{collection_list::CollectionList, requests_list::RequestsList, right::RightState},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum Pane {
    #[default]
    RequestList,
    Request,
    Response,
    CollectionList,
    Relative(RelativePane),
}

#[derive(Debug, Clone, Copy)]
pub enum RelativePane {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct App<'r> {
    pub requests_list: RequestsList<&'r str>,
    pub collection_list: CollectionList<&'r str>,
    pub right_state: RightState,
    pub active_pane: Pane,
    pub last_pane: Pane,
}

impl<'r> App<'r> {
    pub fn new() -> App<'r> {
        App {
            requests_list: RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]),
            collection_list: CollectionList::new(vec![
                "Collection 1",
                "Collection 2",
                "Collection 3",
            ]),
            right_state: RightState::default(),
            active_pane: Pane::RequestList,
            last_pane: Pane::RequestList,
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
            Pane::CollectionList => self.collection_list.handle_key(KeyAction::from(key)),
            _ => None,
        } {
            if let Pane::Relative(_) = pane {
            } else {
                self.focus_pane(pane);
            }
        }
    }

    pub fn focus_pane(&mut self, pane: Pane) {
        self.last_pane = self.active_pane;
        self.active_pane = pane;
    }

    #[inline]
    pub fn focus_last_pane(&mut self) {
        // TODO/BUG: Make sure you cannot focus an invisble pane
        std::mem::swap(&mut self.active_pane, &mut self.last_pane);
    }
}
