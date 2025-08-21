use crate::file_explorer::file_explorer_table::ITEM_HEIGHT;
use ratatui::widgets::{ScrollbarState, TableState};

use super::table_info::TableInfo;

pub struct TableListView {
    pub state: TableState,
    pub items: Vec<TableInfo>, // these hold the tables that are loaded with the .db file
    pub scroll_state: ScrollbarState,
    pub index: usize,
}

impl TableListView {
    pub fn new(items: Vec<TableInfo>) -> Self {
        let item_count = items.len();
        let mut scroll_state = ScrollbarState::new(0);
        if item_count > 0 {
            scroll_state = ScrollbarState::new((item_count - 1) * ITEM_HEIGHT);
        }
        Self {
            state: TableState::default().with_selected(0),
            items,
            scroll_state,
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
                self.index -= 1;
            }
            self.state.select(Some(self.index));
            self.scroll_state = self.scroll_state.position(self.index * ITEM_HEIGHT);
        }
    }
}
