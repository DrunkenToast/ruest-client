use super::ui::right::RightState;

#[derive(Debug)]
pub struct App {
    pub requests: bool,
    pub right_state: RightState,
}

impl App {
    pub fn new() -> App {
        App {
            requests: true,
            right_state: RightState::default(),
        }
    }

    pub fn toggle_requests(&mut self) {
        self.requests = !self.requests;
    }
}

