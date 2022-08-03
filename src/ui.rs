use self::requests_list::RequestsList;

use super::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};
pub mod requests_list;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.requests {
            [Constraint::Percentage(10), Constraint::Percentage(90)]
        } else {
            [Constraint::Max(0), Constraint::Percentage(100)]
        })
        .split(f.size());

    if app.requests {
//RequestsList::new().block(
            //Block::default().title("Collection 1").borders(Borders::ALL)
        f.render_widget(
            &app.requests_list,
         chunks[0]);
    }
    
    let block = Block::default().title("right").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
