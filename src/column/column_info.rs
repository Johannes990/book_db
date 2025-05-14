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
    pub fn name_with_metainfo(&self, with_meta: bool) -> String {
        let mut metainfo = self.name.clone();

        if with_meta {
            metainfo.push(' ');
            if self.is_pk {
                metainfo.push_str("[PK]");
            }
            if self.is_unique {
                metainfo.push_str("[UNIQUE]");
            }
            if self.is_not_null {
                metainfo.push_str("[NOT NULL]");
            }
            if self.is_fk {
                let ref_table = self.references_table.as_deref().unwrap_or("Unknown");
                metainfo.push_str(&format!("[FK -> {}]", ref_table).to_string());
            }
        }

        metainfo
    }
}
