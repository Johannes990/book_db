use crate::fex::fex_data::FileExplorerData;
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;
use std::{fs::{self}, io, path::PathBuf};
use chrono::{DateTime, Utc};


pub const ITEM_HEIGHT: usize = 1;

pub struct FileExplorerTable {
    pub state: TableState,
    pub current_path: PathBuf,
    pub items: Vec<FileExplorerData>,
    pub longest_item_lens: (u16, u16, u16),
    pub scroll_state: ScrollbarState,
    pub index: usize,
}

impl FileExplorerTable {
    pub fn new() -> Self {
        let initial_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let path_data = get_data_from_path(&initial_path);
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((path_data.len() - 1) * ITEM_HEIGHT),
            longest_item_lens: constraint_len_calculator(&path_data),
            current_path: initial_path,
            items: path_data,
            index: 0,
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

    pub fn update_file_list(&mut self) -> io::Result<()> {
        let mut items = get_data_from_path(&self.current_path);
        
        if let Some(_) = self.current_path.parent() {
            items.insert(0, FileExplorerData::new(
                "..".to_string(),
                "".to_string(),
                "".to_string(),
                true
            ));
        }
        self.items = get_data_from_path(&self.current_path);
        self.state = TableState::new().with_selected(self.index);
        Ok(())
    }

    pub fn update_scrollbar_state(&mut self) {
        let path_data = get_data_from_path(&self.current_path);
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
    (name_len as u16, size_field_len as u16, creation_timestamp_len as u16)
}

fn get_data_from_path(path: &PathBuf) -> Vec<FileExplorerData> {
    let mut entries = match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let metadata = match fs::metadata(&path) {
                    Ok(meta) => meta,
                    Err(_) => {
                        return FileExplorerData::new(
                            entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into()),
                            "N/A".into(),
                            "N/A".into(),
                            false,
                        )
                    }
                };
                let is_dir = metadata.is_dir();
                let file_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into());
                let file_size = if is_dir {
                    "<DIR>".to_string()
                } else {
                    format!("{} bytes", metadata.len())
                };
                let date_created = match metadata.created() {
                    Ok(system_time) => {
                        let datetime: DateTime<Utc> = system_time.into();
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                    Err(_) => "N/A".to_string(),
                };

                FileExplorerData::new(file_name, file_size, date_created, is_dir)
            })
            .collect(),
        Err(_) => vec![FileExplorerData::new(
            "<Error reading directory>".into(),
            "N/A".into(),
            "N/A".into(),
            false
        )],
    };

    if path.parent().is_some() {
        entries.insert(0, FileExplorerData::new(
            "..".to_string(),
            "".to_string(),
            "".to_string(),
            true
        ));
    }

    entries
}
