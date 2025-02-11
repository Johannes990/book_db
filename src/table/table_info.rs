#[derive(Clone)]
pub struct TableInfo {
    pub name: String,
    pub row_count: u64,
    pub is_view: bool
}
