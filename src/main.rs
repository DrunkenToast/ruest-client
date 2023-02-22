use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use app::{App, InputMode};
use keys::GlobalKeyAction;
use ui::{theme::Theme, ui};

mod app;
mod component;
mod http;
mod keys;
mod pane;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(Theme::default());
    let res = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App<'_>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // TODO: only redraw when an event changed something
        if let Event::Key(key) = event::read()? {
            // Global keys
            if app.active_pane().input_mode() == InputMode::Normal {
                match GlobalKeyAction::from(key) {
                    GlobalKeyAction::Quit => return Ok(()),
                    GlobalKeyAction::ToggleRequestList => app.requests_list.toggle_visible(),
                    GlobalKeyAction::Send => {
                        match app.send_request().await {
                            Ok(res) => {
                                let (resp, time) = res;
                                app.right_state.response_state.time = time;
                                app.right_state.response_state.status_code = resp.status();
                                if let Ok(data) = resp.text().await {
                                    if let Ok(value) =
                                        serde_json::from_str::<serde_json::Value>(&data)
                                    {
                                        app.right_state.response_state.response =
                                            match serde_json::to_string_pretty(&value) {
                                                Ok(data) => data,
                                                Err(_) => todo!(),
                                            }
                                    } else {
                                        app.right_state.response_state.response = data
                                    }
                                }
                            }
                            Err(res) => app.right_state.response_state.response = res,
                        };
                    }
                    GlobalKeyAction::Methods => {
                        app.methods_list.toggle_visible();
                    }
                    _ => app.handle_key_event(key),
                }
            } else {
                app.handle_key_event(key)
            }
        }
    }
}
