use ratatui::widgets::Row;

use crate::{lang::language::AppLanguage, ui::app_styles::AppStyles};

pub trait StyledRow {
    fn to_row(&self, styles: &AppStyles, language: &AppLanguage, index: usize) -> Row<'_>;
}
