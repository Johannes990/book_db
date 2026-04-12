use crate::widgets::new_table::draft::{ColumnDraft, TableDraft};

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ColumnField {
    Name,
    DataType,
    PrimaryKey,
    NotNull,
    Unique,
    ForeignKeyToggle,
    ForeignKeyTable,
    ForeignKeyColumn,
}

#[allow(dead_code)]
impl ColumnField {
    pub fn next(&self, col: &ColumnDraft) -> Self {
        match self {
            ColumnField::Name => ColumnField::DataType,
            ColumnField::DataType => ColumnField::PrimaryKey,
            ColumnField::PrimaryKey => ColumnField::NotNull,
            ColumnField::NotNull => ColumnField::Unique,
            ColumnField::Unique => ColumnField::ForeignKeyToggle,
            ColumnField::ForeignKeyToggle => {
                if col.foreign_key.is_some() {
                    ColumnField::ForeignKeyTable
                } else {
                    ColumnField::Name
                }
            }
            ColumnField::ForeignKeyTable => ColumnField::ForeignKeyColumn,
            ColumnField::ForeignKeyColumn => ColumnField::Name,
        }
    }

    pub fn previous(&self, col: &ColumnDraft) -> Self {
        match self {
            ColumnField::Name => {
                if col.foreign_key.is_some() {
                    ColumnField::ForeignKeyColumn
                } else {
                    ColumnField::Unique
                }
            }
            ColumnField::DataType => ColumnField::Name,
            ColumnField::PrimaryKey => ColumnField::DataType,
            ColumnField::NotNull => ColumnField::PrimaryKey,
            ColumnField::Unique => ColumnField::NotNull,
            ColumnField::ForeignKeyToggle => ColumnField::Unique,
            ColumnField::ForeignKeyTable => ColumnField::ForeignKeyToggle,
            ColumnField::ForeignKeyColumn => ColumnField::ForeignKeyTable,
        }
    }
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum TableField {
    TableName,
    Column(usize, ColumnField),
}

#[allow(dead_code)]
pub struct TableForm {
    pub draft: TableDraft,
    pub selected_field: TableField,
}

#[allow(dead_code)]
impl TableForm {
    pub fn new() -> Self {
        Self {
            draft: TableDraft::new(),
            selected_field: TableField::TableName,
        }
    }

    pub fn next_form_row(&mut self) {
        let col_count = self.draft.columns.len();

        if col_count == 0 {
            self.selected_field = TableField::TableName;
            return;
        }

        match &self.selected_field {
            TableField::TableName => self.selected_field = TableField::Column(0, ColumnField::Name),
            TableField::Column(col_idx, _col_type) => {
                let next_idx = col_idx + 1;
                if next_idx < col_count {
                    self.selected_field = TableField::Column(next_idx, ColumnField::Name);
                } else {
                    self.selected_field = TableField::TableName;
                }
            }
        }
    }

    pub fn previous_form_row(&mut self) {
        let col_count = self.draft.columns.len();

        if col_count == 0 {
            self.selected_field = TableField::TableName;
            return;
        }

        match &self.selected_field {
            TableField::TableName => {
                self.selected_field = TableField::Column(col_count - 1, ColumnField::Name)
            }
            TableField::Column(col_idx, _col_type) => {
                if *col_idx > 0 {
                    self.selected_field = TableField::Column(col_idx - 1, ColumnField::Name);
                } else {
                    self.selected_field = TableField::TableName;
                }
            }
        }
    }

    pub fn previous_form_row_field(&mut self) {
        if let TableField::Column(col_idx, selected_col_field) = &self.selected_field {
            if let Some(col) = self.draft.columns.get(*col_idx) {
                self.selected_field =
                    TableField::Column(*col_idx, selected_col_field.previous(col));
            }
        }
    }

    pub fn next_form_row_field(&mut self) {
        if let TableField::Column(col_idx, selected_col_field) = &self.selected_field {
            if let Some(col) = self.draft.columns.get(*col_idx) {
                self.selected_field = TableField::Column(*col_idx, selected_col_field.next(col));
            }
        }
    }
}
