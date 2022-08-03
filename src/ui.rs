use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::app::App;
use requests_list::RequestsList;
use right::Right;

pub mod requests_list;
mod right;


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.requests {
            [Constraint::Percentage(10), Constraint::Percentage(90)]
        } else {
            [Constraint::Max(0), Constraint::Percentage(100)]
        })
        .split(f.size());

    if app.requests {
        let items: Vec<ListItem> = app
            .requests_list
            .items
            .iter()
            .map(|i| ListItem::new(Spans::from(*i)))
            .collect();

        let title = app.requests_list.selected().unwrap_or("None selected");

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_symbol("> ");
        
        f.render_stateful_widget(items, chunks[0], &mut app.requests_list.state)
    }

    let block = Block::default().title("right").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
