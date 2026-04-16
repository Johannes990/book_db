#[derive(Clone)]
pub struct TextBox {
    pub text_value: String,
    pub cursor_pos: usize,
}

impl TextBox {
    pub fn new(text_value: String) -> Self {
        let cursor_pos = text_value.clone().chars().count();

        Self {
            text_value,
            cursor_pos,
        }
    }

    pub fn default() -> Self {
        Self {
            text_value: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_value.insert(self.cursor_pos, c);
        self.move_cursor_right();
    }

    pub fn pop_char(&mut self) {
        if self.cursor_pos > 0 {
            self.text_value.remove(self.cursor_pos);
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_pos.saturating_add(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text_value.chars().count())
    }
}
