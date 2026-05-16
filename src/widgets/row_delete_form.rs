use crate::widgets::text_box::TextBox;

pub struct RowDeleteForm {
    pub title: String,
    pub cols: Vec<String>,
    pub selected_col: String,
    pub selected_col_idx: usize,
    pub field_value: TextBox,
}

impl RowDeleteForm {
    pub fn from(title: String, cols: Vec<String>, field_value: TextBox) -> Self {
        Self {
            title,
            cols,
            selected_col: String::new(),
            selected_col_idx: 0,
            field_value,
        }
    }

    pub fn next_col(&mut self) {
        if self.cols.is_empty() {
            return;
        }

        self.selected_col_idx += 1;
        if self.selected_col_idx >= self.cols.len() {
            self.selected_col_idx = 0;
        }
        self.selected_col = self.cols[self.selected_col_idx].clone();
    }

    pub fn previous_col(&mut self) {
        if self.cols.is_empty() {
            return;
        }

        if self.selected_col_idx == 0 {
            self.selected_col_idx = self.cols.len() - 1;
        } else {
            self.selected_col_idx -= 1;
        }
        self.selected_col = self.cols[self.selected_col_idx].clone();
    }
}
