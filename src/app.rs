use std::rc::Rc;

use super::ui::{requests_list::RequestsList, right::RightState};
use crossterm::event::KeyEvent;

use crate::{
    keys::KeyAction,
    ui::theme::{GlobalTheme, Theme}, pane::Pane,
};
use crate::ui::right::RightStatePane;

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
    pub theme: GlobalTheme,
    active_pane_type: PaneType,
}

impl<'a> App<'a> {
    pub fn new(theme: Theme) -> App<'a> {
        let theme = Rc::new(theme);
        let requests_list = RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]);
        let right_state = RightState::new(theme.clone());

        let mut app = App {
            requests_list,
            right_state,
            active_pane_type: PaneType::RequestList,
            theme,
        };
        app.active_pane().set_active(true);
        app
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if let Some(action) = self.active_pane().handle_key(KeyAction::from(key)) {
            if let Actions::MoveRelative(dir) = action {
                self.active_pane().set_active(false);
                if let Some(pane) = self.active_pane().relative_pane(dir) {
                    self.active_pane_type = pane;
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
