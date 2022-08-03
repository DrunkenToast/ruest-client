use super::ui::requests_list::RequestsList;

#[derive(Debug)]
pub struct App<'b> {
    pub requests: bool,
    pub requests_list: RequestsList<'b>,
}

impl<'b> App<'b> {
    pub fn new() -> App<'b> {
        App {
            requests: true,
            requests_list: RequestsList::new()
        }
    }

    pub fn toggle_requests(&mut self) {
        self.requests = !self.requests;
    }
}

