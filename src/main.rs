use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use keys::GlobalKeyAction;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use app::App;
use app::Pane;
use ui::ui;

mod app;
mod keys;
mod ui;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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

        if let Event::Key(key) = event::read()? {
            // Global keys
            match GlobalKeyAction::from(key) {
                GlobalKeyAction::Quit => return Ok(()),
                GlobalKeyAction::ToggleRequestList => app.requests_list.toggle_visible(),
                GlobalKeyAction::ToggleCollectionList => {
                    app.collection_list.toggle_visible();
                    if app.collection_list.visible() {
                        app.focus_pane(Pane::CollectionList);
                    } else {
                        app.focus_last_pane();
                    }
                }
                _ => app.handle_key_event(key),
            }
        }
    }
}
