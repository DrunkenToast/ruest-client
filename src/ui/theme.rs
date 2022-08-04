use std::rc::Rc;

use tui::style::{Color, Style};

pub type GlobalTheme = Rc<Theme>;

#[derive(Debug)]
pub struct Theme {
    selected: Color,
    highlighted: Color,
    focused_block: Color,
    disabled: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_tab: Color::Blue,
            focused_block: Color::White,
            unfocused_block: Color::DarkGray,
        }
    }
}

impl Theme {
    pub fn block(&self, focused: bool) -> Style {
        if focused {
            Style::default().fg(self.focused_block)
        }
        else {
            Style::default().fg(self.unfocused_block)
        }
    }
}

