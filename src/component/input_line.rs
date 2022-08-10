use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::{
    layout::Alignment,
    text::{Span, Spans},
    widgets::{Paragraph, StatefulWidget, Widget},
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
    placeholder: String,
    active: bool,
    theme: GlobalTheme,
    input_mode: InputMode,
    cursor_offset: usize,
}

#[derive(Debug)]
pub enum InputResult {
    Changed,
    Accepted,
    Canceled,
    NOOP,
}

impl InputLineState {
    pub fn new(value: String, placeholder: String, theme: GlobalTheme) -> Self {
        Self {
            prev_value: value.clone(),
            value,
            placeholder,
            theme,
            active: false,
            input_mode: InputMode::Normal,
            cursor_offset: 0,
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_offset = 0;
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
        self.cursor_offset = 0;
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
        let spans = match (state.input_mode, state.cursor_offset) {
            (InputMode::Normal, _) => Spans::from(Span::raw(state.value.as_str())),
            (InputMode::Editing, 0) => Spans::from(vec![
                Span::raw(&state.value),
                Span::styled(" ", state.theme.cursor()),
            ]),
            (InputMode::Editing, cursor_offset) => {
                let offset = state.value.len().saturating_sub(cursor_offset);
                let left = &state.value[..offset];
                let cursor = &state.value[offset..=offset];
                let right = &state.value[offset + 1..];

                Spans::from(vec![
                    Span::raw(left),
                    Span::styled(cursor, state.theme.cursor()),
                    Span::raw(right),
                ])
            }
        };

        if state.value.is_empty() && state.input_mode() == InputMode::Normal {
            // Render placeholder
            Paragraph::new(state.placeholder.as_str())
                .style(state.theme.placeholder())
                .render(area, buf);
        } else {
            Paragraph::new(spans)
                .style(state.theme.hostname())
                .alignment(Alignment::Left)
                .render(area, buf);
        }
    }
}

impl Component for InputLineState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        // TODO: Replace with modifies.intersects(...)
        match key_event {
            KeyEvent {
                code: KeyCode::Char(char),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => {
                if self.cursor_offset == 0 {
                    self.value.push(char);
                } else {
                    self.value
                        .insert(self.value.len() - self.cursor_offset, char)
                }
                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                self.prev_value = self.value.clone();
                self.set_input_mode(InputMode::Normal);
                Some(Action::InputResult(InputResult::Accepted))
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                // TODO: maybe a beep sound or flast when this erorrs
                match self.cursor_offset {
                    0 => {
                        _ = self.value.pop();
                    }
                    n if n == self.value.len() => {}
                    n => _ = self.value.remove(self.value.len() - n),
                }

                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
            } => match self.cursor_offset {
                1 => {
                    _ = self.value.pop().is_some();
                    self.cursor_offset = 0;
                    Some(Action::InputResult(InputResult::Changed))
                }
                n if n > 1 => {
                    _ = self.value.remove(self.value.len() - self.cursor_offset);
                    self.cursor_offset -= 1;
                    Some(Action::InputResult(InputResult::Changed))
                }
                _ => Some(Action::InputResult(InputResult::NOOP)),
            },
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                self.value = self.prev_value.clone();
                self.set_input_mode(InputMode::Normal);
                Some(Action::InputResult(InputResult::Canceled))
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.cursor_offset < self.value.len() {
                    self.cursor_offset += 1;
                    Some(Action::InputResult(InputResult::Changed))
                } else {
                    Some(Action::InputResult(InputResult::NOOP))
                }
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.cursor_offset > 0 {
                    self.cursor_offset -= 1;
                    Some(Action::InputResult(InputResult::Changed))
                } else {
                    Some(Action::InputResult(InputResult::NOOP))
                }
            }
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::Home,
                modifiers: KeyModifiers::NONE,
            } => {
                self.cursor_offset = self.value.len();
                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::End,
                modifiers: KeyModifiers::NONE,
            } => {
                self.cursor_offset = 0;
                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::CONTROL,
            } => {
                let offset = self.value.len().saturating_sub(self.cursor_offset);

                let mut value = String::new();
                if let Some(idx) = self.value[..offset].trim_end().rfind(' ') {
                    value.push_str(&self.value[..=idx]);
                }
                value.push_str(&self.value[offset..]);
                self.value = value;

                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                let offset = self.value.len().saturating_sub(self.cursor_offset);

                self.value = self.value[offset..].to_string();

                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                let offset = self.value.len().saturating_sub(self.cursor_offset);

                self.value = self.value[..offset].to_string();
                self.cursor_offset = 0;

                Some(Action::InputResult(InputResult::Changed))
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::ALT,
            } => {
                let offset = self.value.len().saturating_sub(self.cursor_offset);

                self.cursor_offset = if let Some(idx) = self.value[..offset].trim_end().rfind(' ') {
                    self.value.len() - idx
                } else {
                    self.value.len()
                };
                Some(Action::InputResult(InputResult::NOOP))
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
            }
            | KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::ALT,
            } => {
                    // TODO: Move 1 wort to the right
                    todo!("I rly dont know rn how to do this");
            }
            _ => Some(Action::InputResult(InputResult::NOOP)),
        }
    }

    fn input_mode(&self) -> crate::app::InputMode {
        self.input_mode
    }

    fn active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active
    }
}
