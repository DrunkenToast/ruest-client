use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::pane::Pane;

use super::app::App;

use right::Right;

pub mod requests_list;
pub mod right;
pub mod theme;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.requests_list.visible() {
            [Constraint::Percentage(10), Constraint::Percentage(90)]
        } else {
            [Constraint::Max(0), Constraint::Percentage(100)]
        })
        .split(f.size());

    if app.requests_list.visible() {
        let items: Vec<ListItem> = app
            .requests_list
            .items
            .iter()
            .map(|i| ListItem::new(Spans::from(i.as_ref())))
            .collect();

        let title = app.requests_list.selected().unwrap_or("None selected");

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(app.theme.block(app.requests_list.active())),
            )
            .highlight_symbol("> ")
            .highlight_style(app.theme.selected());

        f.render_stateful_widget(items, chunks[0], &mut app.requests_list.state)
    }

    let block = Block::default().title("right").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    f.render_stateful_widget(Right::default(), chunks[1], &mut app.right_state);
}
