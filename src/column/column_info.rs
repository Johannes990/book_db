#[derive(Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub col_type: String,
    pub is_pk: bool,
    pub is_fk: bool,
    pub references_table: Option<String>,
}
