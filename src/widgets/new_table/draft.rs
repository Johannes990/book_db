use std::fmt;

#[allow(dead_code)]
pub struct ForeignKeyDraft {
    pub referenced_table: String,
    pub referenced_column: String,
}

#[allow(dead_code)]
impl ForeignKeyDraft {
    pub fn new() -> Self {
        Self {
            referenced_table: String::new(),
            referenced_column: String::new(),
        }
    }
}

#[allow(dead_code)]
pub enum SqlDataType {
    Integer,
    Real,
    Text,
    Blob,
}

impl fmt::Display for SqlDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            SqlDataType::Integer => "INTEGER",
            SqlDataType::Real => "REAL",
            SqlDataType::Text => "TEXT",
            SqlDataType::Blob => "BLOB",
        };
        write!(f, "{}", type_str)
    }
}

#[allow(dead_code)]
pub struct ColumnDraft {
    pub name: String,
    pub data_type: SqlDataType,
    pub primary_key: bool,
    pub unique: bool,
    pub not_null: bool,
    pub foreign_key: Option<ForeignKeyDraft>,
}

#[allow(dead_code)]
impl ColumnDraft {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            data_type: SqlDataType::Integer,
            primary_key: false,
            unique: false,
            not_null: false,
            foreign_key: None,
        }
    }

    pub fn toggle_data_type_next(&mut self) {
        self.data_type = match self.data_type {
            SqlDataType::Integer => SqlDataType::Real,
            SqlDataType::Real => SqlDataType::Text,
            SqlDataType::Text => SqlDataType::Blob,
            SqlDataType::Blob => SqlDataType::Integer,
        }
    }

    pub fn toggle_data_type_previous(&mut self) {
        self.data_type = match self.data_type {
            SqlDataType::Integer => SqlDataType::Blob,
            SqlDataType::Real => SqlDataType::Integer,
            SqlDataType::Text => SqlDataType::Real,
            SqlDataType::Blob => SqlDataType::Text,
        }
    }

    pub fn toggle_primary_key(&mut self) {
        self.primary_key = !self.primary_key;
    }

    pub fn toggle_unique(&mut self) {
        self.unique = !self.unique;
    }

    pub fn toggle_not_null(&mut self) {
        self.not_null = !self.not_null;
    }

    pub fn toggle_foreign_key(&mut self) {
        if self.foreign_key.is_some() {
            self.foreign_key = None;
        } else {
            self.foreign_key = Some(ForeignKeyDraft::new());
        }
    }
}

pub struct TableDraft {
    pub name: String,
    pub columns: Vec<ColumnDraft>,
}

#[allow(dead_code)]
impl TableDraft {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            columns: Vec::new(),
        }
    }

    pub fn add_column(&mut self) {
        self.columns.push(ColumnDraft::new());
    }

    pub fn remove_column(&mut self, selected_idx: usize) {
        if !self.columns.is_empty() {
            self.columns.remove(selected_idx);
        }
    }

    pub fn to_sql(&self) -> String {
        let mut col_sql_strings = vec![];

        for col in &self.columns {
            let mut col_def = format!("{} {}", col.name, col.data_type);

            if col.primary_key {
                col_def.push_str(" PRIMARY KEY");
            }

            if col.unique {
                col_def.push_str(" UNIQUE");
            }

            if col.not_null {
                col_def.push_str(" NOT NULL");
            }

            col_sql_strings.push(col_def);
        }

        for col in &self.columns {
            if let Some(fk) = &col.foreign_key {
                let fk_def = format!(
                    "FOREIGN KEY ({}) REFERENCES {}({})",
                    col.name, fk.referenced_table, fk.referenced_column
                );
                col_sql_strings.push(fk_def);
            }
        }

        format!(
            "CREATE TABLE {} (\n{}\n);",
            self.name,
            col_sql_strings.join(",\n")
        )
    }
}
