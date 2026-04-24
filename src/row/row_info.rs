use crate::lang::language::AppLanguage;
use crate::traits::styled_row::StyledRow;
use crate::ui::app_styles::AppStyles;
use ratatui::widgets::Row;

#[derive(Clone)]
pub struct RowInfo {
    pub values: Vec<String>,
}

impl StyledRow for RowInfo {
    fn to_row(&self, styles: &AppStyles, _language: &AppLanguage, index: usize) -> Row<'_> {
        let style = if index % 2 == 0 {
            styles.list_row_style
        } else {
            styles.list_row_alt_style
        };
        Row::new(self.values.clone()).style(style)
    }
}
