use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    layout::Alignment,
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    app::{Action, InputMode},
    ui::theme::GlobalTheme,
};

use super::Component;

#[derive(Debug)]
pub struct InputLineState {
    pub value: String,
    prev_value: String,
    active: bool,
    theme: GlobalTheme,
    input_mode: InputMode,
}

#[derive(Debug)]
pub enum InputResult {
    Changed,
    Accepted,
    Canceled,
    NOOP,
}

impl InputLineState {
    pub fn new(value: String, theme: GlobalTheme) -> Self {
        Self {
            prev_value: value.clone(),
            value,
            theme,
            active: false,
            input_mode: InputMode::Normal,
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode
    }
}

#[derive(Default)]
pub struct InputLine;

impl StatefulWidget for InputLine {
    type State = InputLineState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        Paragraph::new(state.value.clone())
            .style(state.theme.hostname(state.input_mode()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

impl Component for InputLineState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Enter => {
                self.prev_value = self.value.clone();
                self.set_input_mode(InputMode::Normal);
                Some(Action::InputResult(InputResult::Accepted))
            }
            KeyCode::Backspace => {
                // TODO: maybe a beep sound or flast when this erorrs
                _ = self.value.pop().is_some();
                Some(Action::InputResult(InputResult::Changed))
            }
            KeyCode::Esc => {
                self.value = self.prev_value.clone();
                self.set_input_mode(InputMode::Normal);
                Some(Action::InputResult(InputResult::Canceled))
            }
            KeyCode::Char(char) => {
                self.value.push(char);
                Some(Action::InputResult(InputResult::Changed))
            }
            _ => Some(Action::InputResult(InputResult::NOOP)),
        }
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }

    fn input_mode(&self) -> crate::app::InputMode {
        self.input_mode
    }
}
