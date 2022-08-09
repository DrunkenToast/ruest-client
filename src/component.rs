use crossterm::event::KeyEvent;

use crate::{
    app::{Action, InputMode},
    keys::NormalKeyAction,
};

pub mod input_line;

pub trait Component {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        NormalKeyAction::from(key_event).relative_or_none()
    }

    #[inline(always)]
    fn input_mode(&self) -> InputMode {
        InputMode::Normal
    }

    fn active(&self) -> bool;

    fn set_active(&mut self, active: bool);
}
