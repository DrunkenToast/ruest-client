use crate::{
    app::{Actions, Movement, PaneType},
    keys::KeyAction,
};

pub trait Pane {
    fn handle_key(&mut self, key: KeyAction) -> Option<Actions> {
        key.relative_or_none()
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
}
