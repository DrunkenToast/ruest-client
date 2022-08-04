use tui::widgets::ListState;

#[derive(Debug, Clone, Default)]
pub struct RequestsList<T: Copy> {
    pub items: Vec<T>,
    pub state: ListState,
    visible: bool,
}

impl<T: Copy> RequestsList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default(),
            visible: true,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        // Reset state for selection and offset
        self.state = ListState::default();
    }

    pub fn selected(&self) -> Option<T> {
        self.state.selected().map(|i| self.items[i])
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// #[derive(Debug, Clone, Default)]
// pub struct RequestsListState {
//     items: Vec<String>
// }
//
// #[derive(Default, Debug)]
// pub struct RequestsList<'b> {
//     block: Option<Block<'b>>,
// }
//
// impl<'b> RequestsList<'b> {
//     pub fn new() -> RequestsList<'b> {
//         RequestsList {
//             block: None,
//         }
//     }
//
//     pub fn block<'a>(mut self, block: Block<'b>) -> Self {
//         self.block = Some(block);
//         self
//     }
// }
//
// impl<'b> StatefulWidget for RequestsList<'b> {
//     type State = RequestsListState;
//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         List::new(
//             state.items.map(ListItem::new).collect()
//         ).block(self.block.unwrap_or_default()).render(area, buf);
//     }
// }
