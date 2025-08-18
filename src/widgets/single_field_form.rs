use ratatui::style::Style;
use super::text_entry_field::TextEntryField;

pub struct SingleFieldForm {
    pub text_field: TextEntryField,
}

impl SingleFieldForm {
    pub fn new() -> Self {
        SingleFieldForm {
            text_field: TextEntryField::default(String::new(), true),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_field.enter_char(c);
    }

    pub fn pop_char(&mut self) {
        self.text_field.pop_char();
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.text_field.set_on_style(style);
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.text_field.set_off_style(style);
    }
}
