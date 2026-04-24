use crate::lang::language::AppLanguage;
use crate::traits::styled_row::StyledRow;
use crate::ui::app_styles::AppStyles;
use ratatui::widgets::Row;

#[derive(Clone)]
pub struct TableInfo {
    pub name: String,
    pub row_count: u64,
    pub is_view: bool,
}

impl TableInfo {
    pub fn increment_row_count(&mut self) {
        self.row_count += 1;
    }

    pub fn decrement_row_count(&mut self) {
        if self.row_count > 0 {
            self.row_count -= 1;
        }
    }
}

impl StyledRow for TableInfo {
    fn to_row(&self, styles: &AppStyles, language: &AppLanguage, index: usize) -> Row<'_> {
        let style = if index % 2 == 0 {
            styles.list_row_style
        } else {
            styles.list_row_alt_style
        };
        let view_str = if self.is_view {
            language.table_list_view_element.as_str()
        } else {
            language.table_list_table_element.as_str()
        };
        Row::new(vec![
            self.name.clone(),
            self.row_count.to_string(),
            view_str.to_string(),
        ])
        .style(style)
    }
}
