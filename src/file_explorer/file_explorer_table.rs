use crate::{
    file_explorer::file_explorer_data::FileExplorerData,
    log::log,
    threading::{spawn_lookup_thread, spawn_tree_builder},
};
use chrono::{DateTime, Utc};
use ratatui::widgets::{ScrollbarState, TableState};
use std::{
    collections::VecDeque,
    fs::{self},
    path::PathBuf,
    sync::Arc,
    time::Instant,
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

    pub fn parent_path(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.update_file_list();
            self.update_scrollbar_state();
        }
    }

    pub fn open_dir(&mut self, new_path: PathBuf) {
        self.current_path = new_path;
        self.index = 0;
        self.update_file_list();
        self.update_scrollbar_state();
    }

    pub fn update_file_list(&mut self) {
        self.items = get_data_from_path(&self.current_path, &self.language_bits);
        self.state = TableState::new().with_selected(self.index);
    }

    pub fn update_scrollbar_state(&mut self) {
        let item_count = self.items.len();
        self.scroll_state = ScrollbarState::new((item_count - 1) * ITEM_HEIGHT);
    }
}

fn constraint_len_calculator(items: &[FileExplorerData]) -> (u16, u16, u16) {
    let mut name_len = 0;
    let mut size_field_len = 0;
    let mut creation_timestamp_len = 0;

    for item in items {
        name_len = name_len.max(UnicodeWidthStr::width(item.path_name()));
        size_field_len = size_field_len.max(
            item.path_size()
                .lines()
                .map(UnicodeWidthStr::width)
                .max()
                .unwrap_or(0),
        );
        creation_timestamp_len =
            creation_timestamp_len.max(UnicodeWidthStr::width(item.date_created()));
    }

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
    let mut paths_to_search_from: VecDeque<Arc<str>> = VecDeque::new();

    let mut entries = match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .into_string()
                    .unwrap_or_else(|_| invalid_utf_8_string.clone());

                paths_to_search_from.push_front(file_name.clone().into());

                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(_) => {
                        return FileExplorerData::new(
                            file_name,
                            not_available_string.clone(),
                            not_available_string.clone(),
                            false,
                        );
                    }
                };

                let is_dir = metadata.is_dir();

                let file_size = if is_dir {
                    format!("<{}>", dir_string)
                } else {
                    format!("{} {}", metadata.len(), bytes_string)
                };

                let date_created = match metadata.created() {
                    Ok(system_time) => {
                        let datetime: DateTime<Utc> = system_time.into();
                        datetime.format(datetime_format_string).to_string()
                    }
                    Err(_) => not_available_string.clone(),
                };

                FileExplorerData::new(file_name, file_size, date_created, is_dir)
            })
            .collect(),
        Err(_) => vec![FileExplorerData::new(
            format!("<{}>", dir_reading_error),
            not_available_string.clone(),
            not_available_string.clone(),
            false,
        )],
    };

    if path.parent().is_some() {
        entries.insert(
            0,
            FileExplorerData::new("..".into(), "".into(), "".into(), true),
        );
    }

    let start = Instant::now();
    let tree_rx = spawn_tree_builder(paths_to_search_from);
    let tree = tree_rx.recv().unwrap();

    let duration = start.elapsed();
    log(format!("Tree build took {:?}", duration).as_str());

    let start2 = Instant::now();
    let q = spawn_lookup_thread(tree.clone(), Arc::<str>::from("isaac"), 10);
    let duration2 = start2.elapsed();

    log(format!(
        "Query results from 'isaac' in current dir: {:?}",
        q.recv().unwrap()
    )
    .as_str());
    log(format!("querying took {:?}", duration2).as_str());

    entries
}
