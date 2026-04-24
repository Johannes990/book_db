use std::fmt;

use crate::{
    lang::language::AppLanguage, traits::styled_row::StyledRow, ui::app_styles::AppStyles,
    widgets::text_box::TextBox,
};
use ratatui::widgets::Row;

pub struct ForeignKeyDraft {
    pub referenced_table: TextBox,
    pub referenced_column: TextBox,
}

impl ForeignKeyDraft {
    pub fn new() -> Self {
        Self {
            referenced_table: TextBox::default(),
            referenced_column: TextBox::default(),
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

    pub fn toggle_foreign_key(&mut self) {
        if self.foreign_key.is_some() {
            self.foreign_key = None;
        } else {
            self.foreign_key = Some(ForeignKeyDraft::new());
        }
    }
}

impl StyledRow for ColumnDraft {
    fn to_row(&self, styles: &AppStyles, _language: &AppLanguage, index: usize) -> Row<'_> {
        let style = if index % 2 == 0 {
            styles.list_row_style
        } else {
            styles.list_row_alt_style
        };

        let dt = format!("{}", self.data_type);
        let pk = format!("{}", if self.primary_key { "X" } else { "" });
        let unique = format!("{}", if self.unique { "X" } else { "" });
        let nn = format!("{}", if self.not_null { "X" } else { "" });
        let fk = format!("{}", if self.foreign_key.is_some() { "X" } else { "" });
        let mut col_row = vec![self.name.text_value.clone(), dt, pk, unique, nn, fk];

        if let Some(fk) = &self.foreign_key {
            let fk_col = fk.referenced_column.text_value.clone();
            let fk_table = fk.referenced_table.text_value.clone();
            col_row.push(fk_table);
            col_row.push(fk_col);
        }
        Row::new(col_row).style(style)
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
                    col.name.text_value,
                    fk.referenced_table.text_value,
                    fk.referenced_column.text_value
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
