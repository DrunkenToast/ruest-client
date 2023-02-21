use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

use crate::component::Component;

use super::app::App;

use right::Right;

pub mod methods_list;
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
            .map(|&i| ListItem::new(Spans::from(i)))
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

    if app.methods_list.visible() {
        let items: Vec<ListItem> = app
            .methods_list
            .items
            .iter()
            .map(|i| ListItem::new(Spans::from(i.as_str())))
            .collect();

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Methods")
                    .style(app.theme.block(app.methods_list.active())),
            )
            .highlight_symbol("> ")
            .highlight_style(app.theme.selected());
        let area = centered_rect(30, 60, f.size());
        f.render_widget(Clear, area);
        f.render_stateful_widget(items, area, &mut app.methods_list.state);
    }
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
