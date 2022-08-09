use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use super::app::App;
use requests_list::RequestsList;
use right::Right;

mod requests_list;
mod right;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let theme = Rc::new(Theme::default())
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.requests {
            [Constraint::Percentage(10), Constraint::Percentage(90)]
        } else {
            [Constraint::Max(0), Constraint::Percentage(100)]
        })
        .split(f.size());

    if app.requests {
        let requests_list = RequestsList::default().text(":D".to_string());
        f.render_widget(requests_list, chunks[0]);
    }
    
    f.render_widget(Right::default(), chunks[1]);
}
