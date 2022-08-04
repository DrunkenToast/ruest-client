use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Pane, RelativePane};

pub enum GlobalKeyAction {
    Quit,
    Exit,
    ToggleRequestList,
    Other,
}

pub enum KeyAction {
    Exit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    NextTab,
    PrevTab,
    Accept,
    Other,
}

impl KeyAction {
    pub fn relative_or_none(self) -> Option<Pane> {
        match self {
            Self::MoveLeft => Some(Pane::Relative(RelativePane::Left)),
            Self::MoveRight => Some(Pane::Relative(RelativePane::Right)),
            Self::MoveUp => Some(Pane::Relative(RelativePane::Up)),
            Self::MoveDown => Some(Pane::Relative(RelativePane::Down)),
            _ => None,
        }
    }
}
impl From<KeyEvent> for GlobalKeyAction {
    fn from(k: KeyEvent) -> Self {
        match k {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            } => Self::Quit,

            KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::NONE,
            } => Self::ToggleRequestList,

            _ => Self::Other,
        }
    }
}

impl From<KeyEvent> for KeyAction {
    fn from(k: KeyEvent) -> Self {
        match k {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => Self::Exit,

            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveLeft,

            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveRight,

            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveUp,

            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveDown,

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => Self::PrevTab,

            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => Self::NextTab,

            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
            } => Self::Accept,

            _ => Self::Other,
        }
    }
}

// impl From<KeyCode> for KeyActions {
//     fn from(k: KeyCode) -> Self {
//         match k {
//             KeyCode::Char('q') => Self::Quit,
//             KeyCode::Esc => Self::Exit,
//
//             KeyCode::Left | KeyCode::Char('l') => Self::MoveLeft,
//             KeyCode::Right | KeyCode::Char('h') => Self::MoveRight,
//             KeyCode::Up | KeyCode::Char('k') => Self::MoveUp,
//             KeyCode::Down | KeyCode::Char('j') => Self::MoveDown,
//
//             KeyCode::BackTab => Self::PrevTab,
//             KeyCode::Tab => Self::NextTab,
//
//             KeyCode::Enter | KeyCode::Char(' ') => Self::Accept,
//
//             KeyCode::Char('r') => Self::ToggleRequestList,
//             _ => Self::Other,
//         }
//     }
// }
//
