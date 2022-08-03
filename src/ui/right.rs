use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

<<<<<<< HEAD
use self::request::Request;
use self::response::Response;
=======
use self::request::{Request, RequestState};

>>>>>>> 1d0230b0e84c5676f1104d2871313d3c9a85c0fd
mod request;
mod response;

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
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        let block1 = Block::default().title("Request").borders(Borders::ALL);
        let block2 = Block::default().title("Response").borders(Borders::ALL);

<<<<<<< HEAD
        //Request::default().render(chunks[0], buf);
        Request::default().render(block1.inner(chunks[0]), buf);
        Response::default().render(block2.inner(chunks[1]), buf);
=======
        StatefulWidget::render(Request::default(), chunks[0], buf, &mut state.request_state);

>>>>>>> 1d0230b0e84c5676f1104d2871313d3c9a85c0fd
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
