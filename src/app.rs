use super::ui::{requests_list::RequestsList, right::RightState};

#[derive(Debug)]
pub struct App<'r> {
    pub requests: bool,
    pub requests_list: RequestsList<&'r str>,
    pub right_state: RightState,
}

impl<'r> App<'r> {
    pub fn new() -> App<'r> {
        App {
            requests: true,
            requests_list: RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]),
            right_state: RightState::default(),
        }
    }

    pub fn toggle_requests(&mut self) {
        self.requests = !self.requests;
    }
}
