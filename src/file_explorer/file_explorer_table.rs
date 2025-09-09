use crate::file_explorer::file_explorer_data::FileExplorerData;
use chrono::{DateTime, Utc};
use ratatui::widgets::{ScrollbarState, TableState};
use std::{
    fs::{self},
    path::PathBuf,
};
use unicode_width::UnicodeWidthStr;

pub const ITEM_HEIGHT: usize = 1;

pub struct FileExplorerTable {
    pub state: TableState,
    pub current_path: PathBuf,
    pub items: Vec<FileExplorerData>,
    pub longest_item_lens: (u16, u16, u16),
    pub scroll_state: ScrollbarState,
    pub index: usize,
    pub language_bits: (String, String, String, String, String, String),
}

impl FileExplorerTable {
    pub fn new(language_bits: (String, String, String, String, String, String)) -> Self {
        let initial_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let path_data = get_data_from_path(&initial_path, &language_bits);
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((path_data.len() - 1) * ITEM_HEIGHT),
            longest_item_lens: constraint_len_calculator(&path_data),
            current_path: initial_path,
            items: path_data,
            index: 0,
            language_bits,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.index = i;
        self.state.select(Some(self.index));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.index = i;
        self.state.select(Some(self.index));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn update_file_list(&mut self) {
        let mut items = get_data_from_path(&self.current_path, &self.language_bits);

        if self.current_path.parent().is_some() {
            items.insert(
                0,
                FileExplorerData::new("..".to_string(), "".to_string(), "".to_string(), true),
            );
        }
        self.items = get_data_from_path(&self.current_path, &self.language_bits);
        self.state = TableState::new().with_selected(self.index);
    }

    pub fn update_scrollbar_state(&mut self) {
        let path_data = get_data_from_path(&self.current_path, &self.language_bits);
        self.scroll_state = ScrollbarState::new((path_data.len() - 1) * ITEM_HEIGHT);
    }
}

fn constraint_len_calculator(items: &[FileExplorerData]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(FileExplorerData::path_name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let size_field_len = items
        .iter()
        .map(FileExplorerData::path_size)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let creation_timestamp_len = items
        .iter()
        .map(FileExplorerData::date_created)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (
        name_len as u16,
        size_field_len as u16,
        creation_timestamp_len as u16,
    )
}

fn get_data_from_path(
    path: &PathBuf,
    language_bits: &(String, String, String, String, String, String),
) -> Vec<FileExplorerData> {
    let invalid_utf_8_string = &language_bits.0;
    let not_available_string = &language_bits.1;
    let dir_string = &language_bits.2;
    let bytes_string = &language_bits.3;
    let datetime_format_string = &language_bits.4;
    let dir_reading_error = &language_bits.5;

    let mut entries = match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let metadata = match fs::metadata(&path) {
                    Ok(meta) => meta,
                    Err(_) => {
                        return FileExplorerData::new(
                            entry
                                .file_name()
                                .into_string()
                                .unwrap_or_else(|_| invalid_utf_8_string.into()),
                            not_available_string.into(),
                            not_available_string.into(),
                            false,
                        )
                    }
                };
                let is_dir = metadata.is_dir();
                let file_name = entry
                    .file_name()
                    .into_string()
                    .unwrap_or_else(|_| invalid_utf_8_string.into());
                let file_size = if is_dir {
                    format!("<{}>", dir_string)
                } else {
                    format!("{} {}", metadata.len(), bytes_string)
                };
                let date_created = match metadata.created() {
                    Ok(system_time) => {
                        let datetime: DateTime<Utc> = system_time.into();
                        datetime.format(&datetime_format_string).to_string()
                    }
                    Err(_) => not_available_string.to_string(),
                };

                FileExplorerData::new(file_name, file_size, date_created, is_dir)
            })
            .collect(),
        Err(_) => vec![FileExplorerData::new(
            format!("<{}>", dir_reading_error),
            not_available_string.into(),
            not_available_string.into(),
            false,
        )],
    };

    if path.parent().is_some() {
        entries.insert(
            0,
            FileExplorerData::new("..".to_string(), "".to_string(), "".to_string(), true),
        );
    }

    entries
}
