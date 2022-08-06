use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Actions, Movement};

pub enum GlobalKeyAction {
    Quit,
    Exit,
    ToggleRequestList,
    Other,
}

pub enum NormalKeyAction {
    Exit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    NextTab,
    PrevTab,
    Accept,
    InsertMode,
    Other,
}

impl NormalKeyAction {
    pub fn relative_or_none(self) -> Option<Actions> {
        match self {
            Self::MoveLeft => Some(Actions::MoveRelative(Movement::Left)),
            Self::MoveRight => Some(Actions::MoveRelative(Movement::Right)),
            Self::MoveUp => Some(Actions::MoveRelative(Movement::Up)),
            Self::MoveDown => Some(Actions::MoveRelative(Movement::Down)),
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

impl From<KeyEvent> for NormalKeyAction {
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
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
            } => Self::InsertMode,

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
