use ratatui::style::Style;
use super::text_entry_field::TextEntryField;

pub struct DropTableForm {
    pub table_name: TextEntryField,
}

impl DropTableForm {
    pub fn new() -> Self {
        DropTableForm {
            table_name: TextEntryField::default(String::new(), true),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.table_name.enter_char(c);
    }

    pub fn pop_char(&mut self) {
        self.table_name.pop_char();
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.table_name.set_on_style(style);
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.table_name.set_off_style(style);
    }
}
