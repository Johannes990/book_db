use ratatui::{
    style::Style,
    widgets::TableState,
};

use super::text_entry_field::TextEntryField;

#[derive(Clone)]
pub struct TableInsertForm {
    pub state: TableState,
    pub items: Vec<TextEntryField>,
    pub index: usize,
}

impl TableInsertForm {
    pub fn new(col_names: Vec<String>) -> Self {
        let items = col_names.iter().enumerate().map(|(i, _)| {
            TextEntryField::default(String::new(), i == 0)
        }).collect();

        let mut state = TableState::default();
        state.select(Some(0));

        TableInsertForm {
            state,
            items,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.items[self.index].selected = false;
            self.index = (self.index + 1) % self.items.len();
            self.items[self.index].selected = true;
            self.state.select(Some(self.index));
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            self.items[self.index].selected = false;
            self.index = if self.index == 0 {
                self.items.len() - 1
            } else {
                self.index - 1
            };
            self.items[self.index].selected = true;
            self.state.select(Some(self.index));
        }
    }

    pub fn enter_char(&mut self, c: char) {
        if let Some(field) = self.items.get_mut(self.index) {
            field.enter_char(c);
        }
    }

    pub fn pop_char(&mut self) {
        if let Some(field) = self.items.get_mut(self.index) {
            field.pop_char();
        }
    }

    pub fn set_on_style(&mut self, style: Style) {
        if !self.items.is_empty() {
            let _ = self.items.iter_mut().map(|item| item.set_on_style(style));
        }
    }

    pub fn set_off_style(&mut self, style: Style) {
        if !self.items.is_empty() {
            let _ = self.items.iter_mut().map(|item| item.set_off_style(style));
        }
    }
}
