use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
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
mod keys;
mod pane;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(Theme::default());
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // TODO: only redraw when an event changed something
        if let Event::Key(key) = event::read()? {
            // Global keys
            if app.active_pane().input_mode() != InputMode::Editing {
                match GlobalKeyAction::from(key) {
                    GlobalKeyAction::Quit => return Ok(()),
                    GlobalKeyAction::ToggleRequestList => app.requests_list.toggle_visible(),
                    _ => app.handle_key_event(key),
                }
            } else {
                app.handle_key_event(key)
            }
        }
    }
}
