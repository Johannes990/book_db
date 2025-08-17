use ratatui::style::Style;
use super::text_entry_field::TextEntryField;

pub struct CreateDBForm {
    pub file_name: TextEntryField,
}

impl CreateDBForm {
    pub fn new() -> Self {
        CreateDBForm {
            file_name: TextEntryField::default(String::new(), true),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.file_name.enter_char(c);
    }

    pub fn pop_char(&mut self) {
        self.file_name.pop_char();
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.file_name.set_on_style(style);
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.file_name.set_off_style(style);
    }
}
