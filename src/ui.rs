use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

use crate::{app::App, util::centered_rect};

use right::Right;

pub mod collection_list;
pub mod requests_list;
pub mod right;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.requests_list.visible() {
            [Constraint::Percentage(10), Constraint::Percentage(90)]
        } else {
            [Constraint::Max(0), Constraint::Percentage(100)]
        })
        .split(size);

    if app.requests_list.visible() {
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

    f.render_stateful_widget(Right::default(), chunks[1], &mut app.right_state);

    if app.collection_list.visible() {
        let area = centered_rect(60, 20, size);
        let collection_entries: Vec<_> = app
            .collection_list
            .items
            .iter()
            .map(|i| ListItem::new(Spans::from(*i)))
            .collect();

        let title = app.requests_list.selected().unwrap_or("None selected");

        let collection_list = List::new(collection_entries)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_symbol("> ");

        // clear/empty the area of this widget
        f.render_widget(Clear, area);
        f.render_stateful_widget(collection_list, area, &mut app.collection_list.state);
    }
}
