use ratatui::{
    style::Style,
    text::{Line, Span}
};

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
    pub fn get_line_from_col_info(&self, display_metainfo: &bool, col_name_style: Style, metadata_style: Style) -> Line<'static> {
        let mut col_info = vec![Span::styled(self.name.clone(), col_name_style)];
        
        if *display_metainfo {
            col_info.push(Span::raw(" "));
            if self.is_pk {
                col_info.push(Span::styled("[PK]", metadata_style));
            }
            if self.is_unique {
                col_info.push(Span::styled("[UNIQUE]", metadata_style));
            }
            if self.is_not_null {
                col_info.push(Span::styled("[NOT NULL]", metadata_style));
            }
            if self.is_fk {
                let ref_table = self.references_table.as_deref().unwrap_or("Unknown");
                col_info.push(Span::styled(format!("[FK -> {}]", ref_table), metadata_style));
            }
        }

        col_info.push(Span::styled(":", col_name_style));

        Line::from(col_info)
    }

    pub fn col_name_length(&self, display_metainfo: &bool) -> usize {
        let mut length = self.name.len();

        if *display_metainfo{
            length += 1;
            if self.is_pk {
                length += "[PK]".len();
            }
            if self.is_unique {
                length += "[UNIQUE]".len();
            }
            if self.is_not_null {
                length += "[NOT NULL]".len();
            }
            if self.is_fk {
                let ref_table = self.references_table.as_deref().unwrap_or("Unknown");
                length += format!("[FK -> {}]", ref_table).len();
            }
        }

        length
    }
}
