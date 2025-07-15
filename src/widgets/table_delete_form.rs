use ratatui::style::Style;
use super::text_entry_field::TextEntryField;

#[derive(Clone)]
pub struct TableDeleteForm {
    pub col_name_entry: TextEntryField,
    pub row_value_entry: TextEntryField,
}

impl TableDeleteForm {
    pub fn new(col_name: &str, row_value: &str) -> Self {
        let col_name_entry = TextEntryField::default(col_name.to_string(), true);
        let row_value_entry = TextEntryField::default(row_value.to_string(), false);

        TableDeleteForm {
            col_name_entry,
            row_value_entry,
        }
    }

    pub fn switch_field(&mut self) {
        if self.col_name_entry.selected && !(self.row_value_entry.selected) {
            self.col_name_entry.selected = false;
            self.row_value_entry.selected = true;
        } else if self.row_value_entry.selected && !(self.col_name_entry.selected) {
            self.col_name_entry.selected = true;
            self.row_value_entry.selected = false;
        }
    }

    pub fn enter_char(&mut self, c: char) {
        if self.col_name_entry.selected {
            self.col_name_entry.enter_char(c);
        } else if self.row_value_entry.selected {
            self.row_value_entry.enter_char(c);
        }
    }

    pub fn pop_char(&mut self) {
        if self.col_name_entry.selected {
            self.col_name_entry.pop_char();
        } else if self.row_value_entry.selected {
            self.row_value_entry.pop_char();
        }
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.col_name_entry.set_on_style(style);
        self.row_value_entry.set_on_style(style);
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.col_name_entry.set_off_style(style);
        self.row_value_entry.set_off_style(style);
    }
}
