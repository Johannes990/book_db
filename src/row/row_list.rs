use ratatui::widgets::{ScrollbarState, TableState};
use crate::file_explorer::file_explorer_table::ITEM_HEIGHT;
use super::row_info::RowInfo;

pub struct RowListView {
    pub state: TableState,
    pub items: Vec<RowInfo>,
    pub scroll_bar_state: ScrollbarState,
    pub index: usize,
}

impl RowListView {
    pub fn new(items: Vec<RowInfo>) -> Self {
        let item_count = items.len();
        let mut state = TableState::default();

        let scroll_bar_state = if item_count > 0 {
            ScrollbarState::new((item_count - 1) * ITEM_HEIGHT)
        } else {
            ScrollbarState::default()
        };

        if item_count > 0 {
            state.select(Some(0));
        }

        Self {
            state,
            items,
            scroll_bar_state,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.index = (self.index + 1) % self.items.len();
            self.state.select(Some(self.index));
            self.scroll_bar_state = self.scroll_bar_state.position(self.index * ITEM_HEIGHT);
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            if self.index == 0 {
                self.index = self.items.len() - 1;
            } else {
                self.index -= 1;
            }
            self.state.select(Some(self.index));
            self.scroll_bar_state = self.scroll_bar_state.position(self.index * ITEM_HEIGHT);
        }
    }
}
