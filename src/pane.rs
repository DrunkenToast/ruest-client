use crate::{
    app::{Movement, PaneType},
    component::Component,
};

pub trait Pane: Component {
    fn relative_pane(&self, _dir: Movement) -> Option<PaneType> {
        None
    }

    fn active_pane(&mut self, pane: &PaneType) -> &mut dyn Pane;
}
