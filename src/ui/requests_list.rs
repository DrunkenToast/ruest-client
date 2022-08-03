use tui::{
    widgets::{ Widget, List, ListItem, Block },
    layout::Rect,
    style::Style, buffer::Buffer,
};

#[derive(Default, Debug)]
pub struct RequestsList<'b> {
    block: Option<Block<'b>>,
}

impl<'b> RequestsList<'b> {
    pub fn new() -> RequestsList<'b> {
        RequestsList {
            block: None,
        }
    }

    pub fn block<'a>(mut self, block: Block<'b>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'b> Widget for RequestsList<'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        List::new(
            [ListItem::new("Test 1"), ListItem::new("Test 2")]
        ).block(self.block.unwrap_or_default()).render(area, buf);
    }
}
