use std::rc::Rc;

use tui::style::{Color, Modifier, Style};

pub type GlobalTheme = Rc<Theme>;

#[derive(Debug)]
pub struct Theme {
    selected: Color,
    focused: Color,
    disabled: Color,

    status_500_bg: Color,
    status_500_fg: Color,
    status_400_bg: Color,
    status_400_fg: Color,
    status_300_bg: Color,
    status_300_fg: Color,
    status_200_bg: Color,
    status_200_fg: Color,
    status_100_bg: Color,
    status_100_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected: Color::Yellow,
            focused: Color::White,
            disabled: Color::DarkGray,

            status_500_bg: Color::Rgb(255, 62, 62),
            status_500_fg: Color::Rgb(255, 255, 255),
            status_400_bg: Color::Rgb(255, 162, 0),
            status_400_fg: Color::Rgb(255, 255, 255),
            status_300_bg: Color::Rgb(82, 113, 255),
            status_300_fg: Color::Rgb(255, 255, 255),
            status_200_bg: Color::Rgb(53, 183, 41),
            status_200_fg: Color::Rgb(255, 255, 255),
            status_100_bg: Color::Rgb(75, 151, 255),
            status_100_fg: Color::Rgb(255, 255, 255),
        }
    }
}

impl Theme {
    pub fn block(&self, focused: bool) -> Style {
        if focused {
            Style::default().fg(self.focused)
        } else {
            Style::default().fg(self.disabled)
        }
    }

    pub fn selected(&self) -> Style {
        Style::default().fg(self.selected)
    }

    pub fn focused(&self) -> Style {
        Style::default().fg(self.focused)
    }

    pub fn placeholder(&self) -> Style {
        Style::default()
            .add_modifier(Modifier::ITALIC)
            .fg(self.disabled)
    }

    pub fn hostname(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    pub fn cursor(&self) -> Style {
        Style::default().bg(Color::White).fg(Color::Black)
    }

    pub fn status_code(&self, code: u16) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);
        match code {
            c if c / 500 == 1 => style.bg(self.status_500_bg).fg(self.status_500_fg),
            c if c / 400 == 1 => style.bg(self.status_400_bg).fg(self.status_400_fg),
            c if c / 300 == 1 => style.bg(self.status_300_bg).fg(self.status_300_fg),
            c if c / 200 == 1 => style.bg(self.status_200_bg).fg(self.status_200_fg),
            c if c / 100 == 1 => style.bg(self.status_100_bg).fg(self.status_100_fg),
            _ => style.bg(Color::White).fg(Color::Black),
        }
    }
}
