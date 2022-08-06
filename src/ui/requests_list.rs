use tui::widgets::ListState;

use crate::{
    app::{Actions, Movement, PaneType},
    keys::KeyAction,
    pane::Pane,
};

use super::right::RightStatePane;

#[derive(Debug, Default)]
pub struct RequestsList<T: Copy> {
    pub items: Vec<T>,
    pub state: ListState,
    active: bool,
    visible: bool,
}

impl<T: Copy> Pane for RequestsList<T> {
    fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        match key {
            KeyAction::Accept | KeyAction::MoveRight => {
                Some(Actions::MoveRelative(Movement::Right))
            }
            KeyAction::MoveUp => {
                self.previous();
                None
            }
            KeyAction::MoveDown => {
                self.next();
                None
            }
            KeyAction::PrevTab => {
                self.next();
                None
            }
            key => key.relative_or_none(),
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    fn relative_pane(&self, dir: crate::app::Movement) -> Option<PaneType> {
        match dir {
            Movement::Up => None,
            Movement::Down => None,
            Movement::Left => Some(PaneType::Right(RightStatePane::Response)),
            Movement::Right => Some(PaneType::Right(RightStatePane::Request)),
        }
    }
}

impl<T: Copy> RequestsList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default(),
            visible: true,
            active: false,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        // Reset state for selection and offset
        self.state = ListState::default();
    }

    pub fn selected(&self) -> Option<T> {
        self.state.selected().map(|i| self.items[i])
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible
    }
}
