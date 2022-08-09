use crossterm::event::{KeyEvent, KeyCode};

use super::Component;

#[derive(Debug)]
pub struct InputLineComponent {
    pub value: String,
}

pub enum InputResult {
    Changed,
    Accepted,
    Canceled,
    NOOP,
}

impl InputLineComponent {
    pub fn new(value: String) -> Self {
        Self {
            value,
        }
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> InputResult {
        match key_event.code {
            KeyCode::Enter => {
                InputResult::Accepted
            }
            KeyCode::Backspace => {
                // TODO: maybe a beep sound or flast when this erorrs
                _ = self.value.pop().is_some();
                InputResult::Changed
            }
            KeyCode::Esc => {
                InputResult::Canceled
            }
            KeyCode::Char(char) => {
                self.value.push(char);
                InputResult::Changed
            }
            _ => InputResult::NOOP,
        }
    }
}

impl Component for InputLineComponent {}
