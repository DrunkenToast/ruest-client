use tui::widgets::ListState;

use crate::{keys::KeyAction, app::Actions};

use super::super::Pane;

#[derive(Debug, Clone, Default)]
pub struct RequestsList<T: Copy> {
    pub items: Vec<T>,
    pub state: ListState,
    visible: bool,
}

impl<T: Copy> RequestsList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default(),
            visible: true,
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

    pub fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        match key {
            KeyAction::NextTab | KeyAction::Accept | KeyAction::MoveRight => Some(Actions::MoveAbsolute(Pane::Right)),
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

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible
    }
}
