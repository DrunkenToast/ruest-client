use std::cmp;

use crossterm::{event::{KeyCode, KeyEvent, KeyModifiers}, style::Stylize};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
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
    pub fn new(value: String, theme: GlobalTheme) -> Self {
        Self {
            prev_value: value.clone(),
            value,
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
                Span::styled(" ", Style::default().bg(Color::White)),
            ]),
            (InputMode::Editing, cursor_offset) => {
                let offset = state.value.len().saturating_sub(cursor_offset);
                let left = &state.value[..offset];
                let cursor = &state.value[offset..=offset];
                let right = &state.value[offset + 1..];

                Spans::from(vec![
                    Span::raw(left),
                    Span::styled(cursor, Style::default().fg(Color::Black).bg(Color::White)),
                    Span::raw(right),
                ])
            }
        };

        Paragraph::new(spans)
            .alignment(Alignment::Left)
            //.wrap(Wrap { trim: false }) // NOTE: Removes trailing space even with trim off
            .render(area, buf);
    }
}

impl Component for InputLineState {
    fn handle_key(&mut self, key_event: KeyEvent) -> Option<Action> {
        // TODO: Replace with modifies.intersects(...)
        if key_event.modifiers == KeyModifiers::NONE
            || key_event.modifiers == KeyModifiers::SHIFT
        {
            match key_event.code {
                KeyCode::Enter => {
                    self.prev_value = self.value.clone();
                    self.set_input_mode(InputMode::Normal);
                    Some(Action::InputResult(InputResult::Accepted))
                }
                KeyCode::Backspace => {
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
                KeyCode::Delete => match self.cursor_offset {
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
                KeyCode::Esc => {
                    self.value = self.prev_value.clone();
                    self.set_input_mode(InputMode::Normal);
                    Some(Action::InputResult(InputResult::Canceled))
                }
                KeyCode::Char(char) => {
                    if self.cursor_offset == 0 {
                        self.value.push(char);
                    } else {
                        self.value
                            .insert(self.value.len() - self.cursor_offset, char)
                    }
                    Some(Action::InputResult(InputResult::Changed))
                }
                KeyCode::Left => {
                    if self.cursor_offset < self.value.len() {
                        self.cursor_offset += 1;
                        Some(Action::InputResult(InputResult::Changed))
                    } else {
                        Some(Action::InputResult(InputResult::NOOP))
                    }
                }
                KeyCode::Right => {
                    if self.cursor_offset > 0 {
                        self.cursor_offset -= 1;
                        Some(Action::InputResult(InputResult::Changed))
                    } else {
                        Some(Action::InputResult(InputResult::NOOP))
                    }
                }
                _ => Some(Action::InputResult(InputResult::NOOP)),
            }
        } else if key_event.modifiers == KeyModifiers::CONTROL {
            match key_event.code {
                KeyCode::Char('a') => {
                    self.cursor_offset = self.value.len();
                    Some(Action::InputResult(InputResult::Changed))
                }
                KeyCode::Char('e') => {
                    self.cursor_offset = 0;
                    Some(Action::InputResult(InputResult::Changed))
                }
                KeyCode::Char('w') => {
                    let offset = self.value.len().saturating_sub(self.cursor_offset);

                    let mut value = String::new();
                    if let Some(idx) = self.value[..offset].trim_end().rfind(" ") {
                        value.push_str(&self.value[..=idx]);
                    }
                    value.push_str(&self.value[offset..]);
                    self.value = value;

                    Some(Action::InputResult(InputResult::Changed))
                }
                KeyCode::Char('u') => {
                    let offset = self.value.len().saturating_sub(self.cursor_offset);

                    self.value = self.value[offset..].to_string();

                    Some(Action::InputResult(InputResult::Changed))
                }
                KeyCode::Char('k') => {
                    let offset = self.value.len().saturating_sub(self.cursor_offset);

                    self.value = self.value[..offset].to_string();
                    self.cursor_offset = 0;

                    Some(Action::InputResult(InputResult::Changed))
                }

                _ => None,
            }
        } else {
            None
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
