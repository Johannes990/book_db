use ratatui::widgets::{ScrollbarState, TableState};
use crate::fex::fex_table::ITEM_HEIGHT;
use super::column_info::ColumnInfo;

pub struct ColumnListView {
    pub state: TableState,
    pub items: Vec<ColumnInfo>,
    pub scroll_state: ScrollbarState,
    pub index: usize,
}

impl ColumnListView {
    pub fn new(items: Vec<ColumnInfo>) -> Self {
        let item_count = items.len();
        Self {
            state: TableState::default().with_selected(0),
            items,
            scroll_state: ScrollbarState::new((item_count - 1) * ITEM_HEIGHT),
            index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.index = (self.index + 1) % self.items.len();
            self.state.select(Some(self.index));
            self.scroll_state = self.scroll_state.position(self.index * ITEM_HEIGHT);
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            if self.index == 0 {
                self.index = self.items.len() - 1;
            } else {
                self.index = self.index - 1;
            }
            self.state.select(Some(self.index));
            self.scroll_state = self.scroll_state.position(self.index * ITEM_HEIGHT);
        }
    }
}