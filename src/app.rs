use super::ui::requests_list::RequestsList;

#[derive(Debug)]
pub struct App<'r> {
    pub requests: bool,
    pub requests_list: RequestsList<&'r str>,
}

impl<'r> App<'r> {
    pub fn new() -> App<'r> {
        App {
            requests: true,
            requests_list: RequestsList::new(vec![
                "Request 1",
                "Request 2",
                "Request 3",
            ])
        }
    }

    pub fn toggle_requests(&mut self) {
        self.requests = !self.requests;
    }
}

