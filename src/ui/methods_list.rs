use std::sync::{Arc, Mutex};

use tui::widgets::ListState;

use crate::{component::Component, keys::NormalKeyAction};

#[derive(Debug, Default)]
pub struct MethodsList {
    pub items: Vec<reqwest::Method>,
    pub state: ListState,
    active: bool,
    visible: bool,
    selected_method: Arc<Mutex<reqwest::Method>>,
}
impl Component for MethodsList {
    fn handle_key(&mut self, key_event: crossterm::event::KeyEvent) -> Option<crate::app::Action> {
        match NormalKeyAction::from(key_event) {
            NormalKeyAction::Accept | NormalKeyAction::Exit => {
                match self.selected() {
                    Some(method) => {
                        *self.selected_method.lock().unwrap() = method;
                    }
                    None => todo!(),
                };
                self.toggle_visible();
                None
            }
            NormalKeyAction::MoveUp => {
                self.previous();
                None
            }
            NormalKeyAction::MoveDown => {
                self.next();
                None
            }
            _ => todo!(),
        }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}

impl MethodsList {
    pub fn new(items: Vec<reqwest::Method>, selected_method: Arc<Mutex<reqwest::Method>>) -> Self {
        let mut _state = ListState::default();
        let selected = match _state.selected() {
            Some(i) => Some(i),
            None => Some(0),
        };
        _state.select(selected);
        Self {
            items,
            state: _state,
            visible: false,
            active: false,
            selected_method,
        }
    }
    pub fn selected(&self) -> Option<reqwest::Method> {
        self.state.selected().map(|i| self.items[i].clone())
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
