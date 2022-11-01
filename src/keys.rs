use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Action, Movement};

pub enum GlobalKeyAction {
    Quit,
    Exit,
    ToggleRequestList,
    Send,
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
    pub fn relative_or_none(self) -> Option<Action> {
        match self {
            Self::MoveLeft => Some(Action::MoveRelative(Movement::Left)),
            Self::MoveRight => Some(Action::MoveRelative(Movement::Right)),
            Self::MoveUp => Some(Action::MoveRelative(Movement::Up)),
            Self::MoveDown => Some(Action::MoveRelative(Movement::Down)),
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

            KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
            } => Self::Send,

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
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveLeft,

            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveRight,
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
            } => Self::MoveUp,
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('j'),
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
                code: KeyCode::Enter | KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
            } => Self::Accept,
            _ => Self::Other,
        }
    }
}
