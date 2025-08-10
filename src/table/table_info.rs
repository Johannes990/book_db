#[derive(Clone)]
pub struct TableInfo {
    pub name: String,
    pub row_count: u64,
    pub is_view: bool
}

impl TableInfo {
    pub fn increment_row_count(&mut self) {
        self.row_count += 1;
    }

    pub fn decrement_row_count(&mut self) {
        if self.row_count > 0 {
            self.row_count -= 1;
        }
    }
}
