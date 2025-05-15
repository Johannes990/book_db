#[derive(Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub col_type: String,
    pub is_pk: bool,
    pub is_fk: bool,
    pub references_table: Option<String>,
    pub is_unique: bool,
    pub is_not_null: bool,
}

impl ColumnInfo {
    pub fn name_with_metainfo(&self, display_metainfo: &bool) -> String {
        let mut col_string = self.name.clone();

        if *display_metainfo {
            col_string.push(' ');
            if self.is_pk {
                col_string.push_str("[PK]");
            }
            if self.is_unique {
                col_string.push_str("[UNIQUE]");
            }
            if self.is_not_null {
                col_string.push_str("[NOT NULL]");
            }
            if self.is_fk {
                let ref_table = self.references_table.as_deref().unwrap_or("Unknown");
                col_string.push_str(&format!("[FK -> {}]", ref_table).to_string());
            }
        }

        col_string
    }
}
