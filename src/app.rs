#[derive(Debug)]
pub struct App {
    pub requests: bool,
}

impl App {
    pub fn new() -> App {
        App {
            requests: true,
        }
    }

    pub fn toggle_requests(&mut self) {
        self.requests = !self.requests;
    }
}

