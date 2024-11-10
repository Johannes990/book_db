use crate::fexp::fexpdata::FileExplorerData;
use ratatui::widgets::{ScrollbarState, TableState};
use unicode_width::UnicodeWidthStr;
use std::{fs::{self}, io, path::PathBuf};
use chrono::{DateTime, Utc};


const ITEM_HEIGHT: usize = 4;

pub struct FileExplorerTable {
    state: TableState,
    pub current_path: PathBuf,
    items: Vec<FileExplorerData>,
    longest_item_lens: (u16, u16, u16),
    scroll_state: ScrollbarState,
    index: usize,
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
        self.scroll_state = self.scroll_state.position(self.index * ITEM_HEIGHT);
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
        self.scroll_state = self.scroll_state.position(self.index * ITEM_HEIGHT);
    }

    pub fn update_file_list(&mut self) -> io::Result<()> {
        self.items = get_data_from_path(&self.current_path);
        self.index = 0;
        Ok(())
    }
}

fn constraint_len_calculator(items: &[FileExplorerData]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(FileExplorerData::path_name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(FileExplorerData::path_size)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let email_len = items
        .iter()
        .map(FileExplorerData::date_created)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, address_len as u16, email_len as u16)
}

fn get_data_from_path(path: &PathBuf) -> Vec<FileExplorerData> {
    match fs::read_dir(path) {
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
    }
}