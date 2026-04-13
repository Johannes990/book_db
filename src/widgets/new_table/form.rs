use ratatui::widgets::{ScrollbarState, TableState};

use crate::{file_explorer::file_explorer_table::ITEM_HEIGHT, widgets::{new_table::draft::{ColumnDraft, TableDraft}, text_box::TextBox}};

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
pub struct CreateTableForm {
    pub table_state: TableState,
    pub draft: TableDraft,
    pub selected_field: TableField,
    pub scroll_state: ScrollbarState,
}

#[allow(dead_code)]
impl CreateTableForm {
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
            draft: TableDraft::new(),
            selected_field: TableField::TableName,
            scroll_state: ScrollbarState::new(1 * ITEM_HEIGHT),
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

        self.sync_table_state();
    }

    pub fn previous_form_row(&mut self) {
        let col_count = self.draft.columns.len();

        if col_count == 0 {
            self.selected_field = TableField::TableName;
            return;
        }

        match &self.selected_field {
            TableField::TableName => self.selected_field = TableField::Column(col_count - 1, ColumnField::Name),
            TableField::Column(col_idx, _col_type) => {
                if *col_idx > 0 {
                    self.selected_field = TableField::Column(col_idx - 1, ColumnField::Name);
                } else {
                    self.selected_field = TableField::TableName;
                }
            }
        }

        self.sync_table_state();
    }

    pub fn previous_form_row_field(&mut self) {
        if let TableField::Column(col_idx, selected_col_field) = &self.selected_field {
            if let Some(col) = self.draft.columns.get(*col_idx) {
                self.selected_field =
                    TableField::Column(*col_idx, selected_col_field.previous(col));
            }
        };

        
    }

    pub fn next_form_row_field(&mut self) {
        if let TableField::Column(col_idx, selected_col_field) = &self.selected_field {
            if let Some(col) = self.draft.columns.get(*col_idx) {
                self.selected_field = TableField::Column(*col_idx, selected_col_field.next(col));
            }
        }
    }

    pub fn is_table_name_field_selected(&self) -> bool {
        matches!(self.selected_field, TableField::TableName)
    }

    pub fn is_col_name_field_selected(&self, col_idx: usize) -> bool {
        matches!(self.selected_field, TableField::Column(i, ColumnField::Name)if i == col_idx)
    }

    pub fn is_col_fk_table_field_selected(&self, col_idx: usize) -> bool {
        matches!(self.selected_field, TableField::Column(i, ColumnField::ForeignKeyTable) if i == col_idx)
    }

    pub fn is_col_fk_column_field_selected(&self, col_idx: usize) -> bool {
        matches!(self.selected_field, TableField::Column(i, ColumnField::ForeignKeyColumn) if i == col_idx)
    }

    pub fn selected_textbox_mut(&mut self) -> Option<&mut TextBox> {
        match self.selected_field {
            TableField::TableName => Some(&mut self.draft.name),
            TableField::Column(i, ColumnField::ForeignKeyTable) => {
                self.draft.columns.get_mut(i)
                    .and_then(|c| c.foreign_key.as_mut())
                    .map(|fk| &mut fk.referenced_table)
            }
            TableField::Column(i, ColumnField::ForeignKeyColumn) => {
                self.draft.columns.get_mut(i)
                    .and_then(|c| c.foreign_key.as_mut())
                    .map(|fk| &mut fk.referenced_column)
            }
            _ => None,
        }
    }

    fn sync_table_state(&mut self) {
        if let Some(i) = self.selected_row() {
            self.table_state.select(Some(i));
            self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
        } else {
            self.table_state.select(None);
        }
    }

    fn selected_row(&self) -> Option<usize> {
        match self.selected_field {
            TableField::Column(i, _) => Some(i),
            _ => None,
        }
    }
}
