use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::Row,
};

use crate::{
    lang::language::AppLanguage, traits::styled_row::StyledRow, ui::app_styles::AppStyles,
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
    pub fn get_line_from_col_info(
        &self,
        language_strings: (&str, &str, &str, &str, &str),
        display_metainfo: bool,
        col_name_style: Style,
        metadata_style: Style,
    ) -> Line<'static> {
        let mut col_info = vec![Span::styled(self.name.clone(), col_name_style)];

        if display_metainfo {
            col_info.push(Span::raw(" "));
            if self.is_pk {
                col_info.push(Span::styled(
                    format!("[{}]", language_strings.0),
                    metadata_style,
                ));
            }
            if self.is_unique {
                col_info.push(Span::styled(
                    format!("[{}]", language_strings.1),
                    metadata_style,
                ));
            }
            if self.is_not_null {
                col_info.push(Span::styled(
                    format!("[{}]", language_strings.2),
                    metadata_style,
                ));
            }
            if self.is_fk {
                let ref_table = self
                    .references_table
                    .as_deref()
                    .unwrap_or(language_strings.4);
                col_info.push(Span::styled(
                    format!("[{} -> {}]", language_strings.3, ref_table),
                    metadata_style,
                ));
            }
        }

        col_info.push(Span::styled(":", col_name_style));

        Line::from(col_info)
    }

    pub fn col_name_length(
        &self,
        language_strings: (&str, &str, &str, &str, &str),
        display_metainfo: bool,
    ) -> usize {
        let mut length = self.name.len();

        if display_metainfo {
            let angle_brackets_len = 2;
            length += 1;
            if self.is_pk {
                length += angle_brackets_len + language_strings.0.len();
            }
            if self.is_unique {
                length += angle_brackets_len + language_strings.1.len();
            }
            if self.is_not_null {
                length += angle_brackets_len + language_strings.2.len();
            }
            if self.is_fk {
                let ref_table = self
                    .references_table
                    .as_deref()
                    .unwrap_or(language_strings.4);
                length += format!("[{} -> {}]", language_strings.3, ref_table).len();
            }
        }

        length
    }
}

impl StyledRow for ColumnInfo {
    fn to_row(&self, styles: &AppStyles, language: &AppLanguage, index: usize) -> Row<'_> {
        let style = if index % 2 == 0 {
            styles.list_row_style
        } else {
            styles.list_row_alt_style
        };
        let mut col_constraint_text = "".to_string();
        if self.is_pk {
            let pk_string = &language.sql_pk_constraint;
            col_constraint_text.push_str(format!("[{}]", pk_string).as_str());
        }
        if self.is_unique {
            let unique_string = &language.sql_unique_constraint;
            col_constraint_text.push_str(format!("[{}]", unique_string).as_str());
        }
        if self.is_not_null {
            let not_null_string = &language.sql_not_null_constraint;
            col_constraint_text.push_str(format!("[{}]", not_null_string).as_str());
        }
        if self.is_fk {
            let unknown_ref_table_string = &language.column_list_unknown_fk_ref;
            let fk_string = &language.sql_fk_constraint;
            let ref_table = self
                .references_table
                .as_deref()
                .unwrap_or(&unknown_ref_table_string);
            col_constraint_text.push_str(&format!("[{} -> {}]", fk_string, ref_table).to_string());
        }

        Row::new(vec![
            self.name.to_string(),
            self.col_type.to_string(),
            col_constraint_text,
        ])
        .style(style)
    }
}
