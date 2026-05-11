use std::collections::BTreeMap;
use std::fmt;

use crate::widgets::text_box::TextBox;

pub struct ForeignKeyDraft {
    pub referenced_table: String,
    table_idx: usize,
    pub referenced_column: String,
    column_idx: usize,
}

impl ForeignKeyDraft {
    pub fn new(tab_col_map: &BTreeMap<String, Vec<String>>) -> Self {
        let ofkv = tab_col_map.first_key_value();
        match ofkv {
            Some(fkv) => {
                let referenced_table = fkv.0.to_string();
                let referenced_column = fkv.1[0].clone();

                Self {
                    referenced_table,
                    table_idx: 0,
                    referenced_column,
                    column_idx: 0,
                }
            }
            None => Self {
                referenced_table: String::new(),
                table_idx: 0,
                referenced_column: String::new(),
                column_idx: 0,
            },
        }
    }
}

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

pub struct ColumnDraft {
    pub name: TextBox,
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
            name: TextBox::default(),
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

    pub fn toggle_foreign_key(&mut self, tab_col_map: &BTreeMap<String, Vec<String>>) {
        if self.foreign_key.is_some() {
            self.foreign_key = None;
        } else {
            self.foreign_key = Some(ForeignKeyDraft::new(tab_col_map));
        }
    }

    pub fn toggle_foreign_key_table(&mut self, tab_col_map: &BTreeMap<String, Vec<String>>) {
        if tab_col_map.is_empty() {
            return;
        }

        let Some(fk_field) = self.foreign_key.as_mut() else {
            return;
        };

        let tables: Vec<_> = tab_col_map.keys().collect();
        fk_field.table_idx += 1;

        if fk_field.table_idx >= tab_col_map.len() {
            fk_field.table_idx = 0;
        }

        fk_field.referenced_table = tables[fk_field.table_idx].to_string();

        let columns = match tab_col_map.get(&fk_field.referenced_table) {
            Some(column_vec) => column_vec,
            None => &Vec::new(),
        };

        if columns.len() == 0 {
            fk_field.referenced_column = "".to_string();
        }

        if fk_field.column_idx >= columns.len() {
            fk_field.column_idx = 0;
        }

        fk_field.referenced_column = columns[fk_field.column_idx].clone();
    }

    pub fn toggle_foreign_key_column(&mut self, tab_col_map: &BTreeMap<String, Vec<String>>) {
        if tab_col_map.is_empty() {
            return;
        }

        let Some(fk_field) = self.foreign_key.as_mut() else {
            return;
        };

        let columns = match tab_col_map.get(&fk_field.referenced_table) {
            Some(column_vec) => column_vec,
            None => &Vec::new(),
        };

        fk_field.column_idx += 1;
        if fk_field.column_idx >= columns.len() {
            fk_field.column_idx = 0;
        }

        fk_field.referenced_column = columns[fk_field.column_idx].clone();
    }
}

pub struct TableDraft {
    pub name: TextBox,
    pub columns: Vec<ColumnDraft>,
}

#[allow(dead_code)]
impl TableDraft {
    pub fn new() -> Self {
        Self {
            name: TextBox::default(),
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
            let mut col_def = format!("{} {}", col.name.text_value, col.data_type);

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
                    col.name.text_value, fk.referenced_table, fk.referenced_column
                );
                col_sql_strings.push(fk_def);
            }
        }

        format!(
            "CREATE TABLE {} (\n{}\n);",
            self.name.text_value,
            col_sql_strings.join(",\n")
        )
    }
}
