use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use self::request::{Request, RequestState};

mod request;

#[derive(Debug, Default)]
pub struct RightState {
    pub request_state: RequestState,
}

#[derive(Default)]
pub struct Right;

impl StatefulWidget for Right {
    type State = RightState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(area);

        let block1 = Block::default().title("Request").borders(Borders::ALL);
        let block2 = Block::default().title("Response").borders(Borders::ALL);

        StatefulWidget::render(Request::default(), chunks[0], buf, &mut state.request_state);

        block1.render(chunks[0], buf);
        block2.render(chunks[1], buf);
    }
}

impl Widget for Right {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = RightState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
