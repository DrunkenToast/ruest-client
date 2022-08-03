use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use self::request::Request;

mod request;

#[derive(Default)]
pub struct Right;

impl Widget for Right {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(area);

        let block1 = Block::default().title("Request").borders(Borders::ALL);
        let block2 = Block::default().title("Response").borders(Borders::ALL);

        //Request::default().render(chunks[0], buf);
        Request::default().render(block1.inner(chunks[0]), buf);

        block1.render(chunks[0], buf);
        block2.render(chunks[1], buf);
    }
}
