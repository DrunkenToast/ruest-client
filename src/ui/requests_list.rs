use tui::widgets::ListState;
use crate::keys::KeyAction;

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
        match self.state.selected() {
            Some(i) => Some(self.items[i]),
            None => None,
        }
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

    pub fn handle_key(&mut self, key: KeyAction) -> Option<Pane> {
        match key {
            KeyAction::NextTab | KeyAction::Accept | KeyAction::MoveRight =>
                // Also select request
                Some(Pane::Request),
            KeyAction::MoveUp => {
                self.previous();
                None
            },
            KeyAction::MoveDown => {
                self.next();
                None
            },
            _ => None
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible
    }
}

