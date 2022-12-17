use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::ui::{requests_list::RequestsList, right::RightState};
use crate::{
    component::{input_line::InputResult, Component},
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

    /// Only keybinds for existing insert mode are active
    Editing,
}

#[derive(Debug)]
pub enum Action {
    MoveRelative(Movement),
    InputResult(InputResult),
}

pub struct App<'a> {
    pub requests_list: RequestsList<&'a str>,
    pub right_state: RightState,
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
            if let Action::MoveRelative(dir) = action {
                // TODO: move .relative_pane() into .handle_key()
                if let Some(pane) = self.active_pane().relative_pane(dir) {
                    self.activate_pane(pane);
                }
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
    pub async fn send_request(&mut self) -> Response {
        let method = reqwest::Method::GET;
        let uri = &self.right_state.request_state.hostname_input_state.value;
        let resp = http_request(
            method,
            uri,
            HeaderMap::new(),
            HeaderValue::from_str("").unwrap(),
            "{}",
        )
        .await;
        let response = match resp {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        response
    }
}
