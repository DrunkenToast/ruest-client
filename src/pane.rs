use crossterm::event::KeyEvent;

use crate::{
    app::{Actions, InputMode, Movement, PaneType},
    keys::NormalKeyAction,
};

pub trait Pane {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Actions> {
        NormalKeyAction::from(key_event).relative_or_none()
    }

    fn relative_pane(&self, dir: Movement) -> Option<PaneType> {
        match dir {
            Movement::Up => None,
            Movement::Down => None,
            Movement::Left => None,
            Movement::Right => None,
        }
    }

    fn active_pane(&mut self, pane: &PaneType) -> &mut dyn Pane;

    fn active(&self) -> bool;

    fn set_active(&mut self, active: bool);

    #[inline(always)]
    fn input_mode(&self) -> InputMode {
        InputMode::Normal
    }
}
