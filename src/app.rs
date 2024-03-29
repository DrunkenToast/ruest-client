use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};

use super::ui::{requests_list::RequestsList, right::RightState};
use crate::{
    component::Component,
    http::http_request,
    pane::Pane,
    ui::{
        methods_list::MethodsList,
        right::RightStatePane,
        theme::{GlobalTheme, Theme},
    },
};
use crossterm::event::KeyEvent;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PaneType {
    #[default]
    RequestList,
    Right(RightStatePane),
}

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

/// Possible input modes
/// these mode are used to determine wich keybinds are active
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputMode {
    /// All "normal" keybinds are active
    Normal,
    /// Only keybinds for editing mode are active
    Hostname,
    Body,
}

#[derive(Debug)]
pub enum Action {
    MoveRelative(Movement),
}

pub struct App<'a> {
    pub requests_list: RequestsList<&'a str>,
    pub right_state: RightState<'a>,
    pub theme: GlobalTheme,
    active_pane_type: PaneType,
    pub methods_list: MethodsList,
}

impl<'a> App<'a> {
    pub fn new(theme: Theme) -> App<'a> {
        let theme = Rc::new(theme);
        let requests_list = RequestsList::new(vec!["Request 1", "Request 2", "Request 3"]);
        let selected_method = Arc::new(Mutex::new(reqwest::Method::GET));
        let methods_list = MethodsList::new(
            vec![
                reqwest::Method::GET,
                reqwest::Method::POST,
                reqwest::Method::PUT,
                reqwest::Method::DELETE,
                reqwest::Method::PATCH,
                reqwest::Method::HEAD,
                reqwest::Method::OPTIONS,
            ],
            selected_method.clone(),
        );
        let right_state = RightState::new(theme.clone(), selected_method.clone());

        let mut app = App {
            requests_list,
            right_state,
            active_pane_type: PaneType::RequestList,
            theme,
            methods_list,
        };
        app.active_pane().set_active(true);
        app
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.methods_list.visible() {
            self.methods_list.handle_key(key_event);
            return;
        }
        if let Some(action) = self.active_pane().handle_key(key_event) {
            match action {
                Action::MoveRelative(dir) => {
                    // TODO: move .relative_pane() into .handle_key()
                    if let Some(pane) = self.active_pane().relative_pane(dir) {
                        self.activate_pane(pane);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn active_pane(&mut self) -> &mut dyn Pane {
        match self.active_pane_type {
            PaneType::RequestList => self.requests_list.active_pane(&self.active_pane_type),
            PaneType::Right(_) => self.right_state.active_pane(&self.active_pane_type),
        }
    }

    fn activate_pane(&mut self, pane: PaneType) {
        if self.active_pane_type != pane {
            self.active_pane().set_active(false);
            self.active_pane_type = pane;
            self.active_pane().set_active(true);
        }
    }
    pub async fn send_request(&mut self) -> Result<(Response, Duration), String> {
        match self.methods_list.selected() {
            Some(method) => {
                let uri = self
                    .right_state
                    .request_state
                    .input_line
                    .clone()
                    .into_lines()
                    .join("\n");

                let body = self
                    .right_state
                    .request_state
                    .body
                    .clone()
                    .into_lines()
                    .join("\n");

                let resp = http_request(
                    method,
                    uri,
                    HeaderMap::new(),
                    HeaderValue::from_str("application/json").unwrap(),
                    body,
                )
                .await;
                match resp {
                    Ok(r) => Ok(r),
                    Err(_e) => Err(String::from("Invalid or unavailable URI (Make sure to include the url scheme, for example: http://)")),
                }
            }
            _ => panic!("Not a valid method?"),
        }
    }
}
