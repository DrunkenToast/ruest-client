use tui::{
    widgets::{ Widget, List, ListItem },
    layout::Rect,
    style::Style, buffer::Buffer,
};

#[derive(Default)]
pub struct RequestsList {
    text: String,
}

impl Widget for RequestsList {
    fn render(self, area: Rect, buf: &mut Buffer){
        buf.set_string(area.left(), area.top(), self.text, Style::default())
    }
}

impl RequestsList {
    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}
