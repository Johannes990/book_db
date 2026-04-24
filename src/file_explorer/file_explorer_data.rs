use crate::lang::language::AppLanguage;
use crate::traits::styled_row::StyledRow;
use crate::ui::app_styles::AppStyles;
use ratatui::widgets::Row;

pub struct FileExplorerData {
    path_name: String,
    path_size: String,
    date_created: String,
    is_dir: bool,
}

impl FileExplorerData {
    pub fn new(path_name: String, path_size: String, date_created: String, is_dir: bool) -> Self {
        Self {
            path_name,
            path_size,
            date_created,
            is_dir,
        }
    }

    pub const fn ref_array(&self) -> [&String; 3] {
        [&self.path_name, &self.path_size, &self.date_created]
    }

    pub fn path_name(&self) -> &str {
        &self.path_name
    }

    pub fn path_size(&self) -> &str {
        &self.path_size
    }

    pub fn date_created(&self) -> &str {
        &self.date_created
    }

    pub fn is_dir(&self) -> &bool {
        &self.is_dir
    }
}

impl StyledRow for FileExplorerData {
    fn to_row(&self, styles: &AppStyles, _language: &AppLanguage, index: usize) -> Row<'_> {
        let style = if index % 2 == 0 {
            styles.list_row_style
        } else {
            styles.list_row_alt_style
        };
        Row::new(self.ref_array().iter().map(|item| item.as_str())).style(style)
    }
}
