use crossterm::event::{KeyEvent};
use tui::{
    layout::Alignment,
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::{Action, InputMode},
    ui::theme::GlobalTheme,
};

use super::Component;

#[derive(Debug)]
pub struct InputBoxState {
    pub value: String,
    placeholder: String,
    theme: GlobalTheme,
    active: bool,
}

impl InputBoxState {
    pub fn new<S>(value: S, placeholder: S, theme: GlobalTheme) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
            placeholder: placeholder.into(),
            theme,
            active: false,
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn edit(&mut self) {
        if let Ok(s) = edit::edit(self.value.as_str()) {
            self.value = s;
        }
    }
}

#[derive(Default)]
pub struct InputBox;

impl StatefulWidget for InputBox {
    type State = InputBoxState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        if state.value.is_empty() && state.input_mode() == InputMode::Normal {
            // Render placeholder
            Paragraph::new(state.placeholder.as_str())
                .style(state.theme.placeholder())
                .render(area, buf);
        } else {
            Paragraph::new(state.value.as_str())
                .alignment(Alignment::Left)
                .render(area, buf);
        }
    }
}

impl Component for InputBoxState {
    fn handle_key(&mut self, _key_event: KeyEvent) -> Option<Action> {
        todo!()
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}
