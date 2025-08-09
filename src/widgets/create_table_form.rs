use ratatui::style::Style;
use super::text_entry_field::TextEntryField;

pub struct CreateTableForm {
    pub sql: TextEntryField,
}

impl CreateTableForm {
    pub fn new() -> Self {
        CreateTableForm {
            sql: TextEntryField::default(String::new(), true),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.sql.enter_char(c);
    }

    pub fn pop_char(&mut self) {
        self.sql.pop_char();
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.sql.set_on_style(style);
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.sql.set_off_style(style);
    }
}
