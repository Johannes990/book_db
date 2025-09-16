use crate::{
    column::{column_info::ColumnInfo, column_list::ColumnListView},
    db::{DBError, DB},
    file_explorer::file_explorer_table::FileExplorerTable,
    handle_key_events,
    lang::language::AppLanguage,
    log::log,
    options::Options,
    perf::{resources::Resources, statistics::StatisticsProfiling},
    row::row_list::RowListView,
    table::{table_info::TableInfo, table_list::TableListView},
    ui::{
        colors::{app_colors::ColorScheme, static_colors::StaticColors},
        input::key_bindings::KeyBindings,
        render,
    },
    widgets::text_form::TextForm,
};
use ratatui::{style::Color, Terminal};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet, ffi::OsString, io, path::PathBuf, sync::mpsc::Receiver, time::Instant,
};
use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Screen {
    Splash,
    FileExplorer,
    DatabaseSchema,
    DataBaseTable,
    CreateNewFile,
    Options,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PopUp {
    None,
    Quit,
    NoDBLoaded,
    InsertRow,
    DeleteRow,
    InsertTable,
    DeleteTable,
    Error,
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Mode {
    Browse,
    Edit,
}

pub struct App {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub current_mode: Mode,
    pub current_error: Option<DBError>,
    pub selected_db: Option<DB>,
    pub selected_db_table: Option<String>,
    pub selected_table_columns: Vec<ColumnInfo>,
    pub file_explorer_table: FileExplorerTable,
    pub table_list_view: Option<TableListView>,
    pub column_list_view: Option<ColumnListView>,
    pub row_list_view: Option<RowListView>,
    pub row_insert_form: Option<TextForm>,
    pub row_delete_form: Option<TextForm>,
    pub table_insert_form: Option<TextForm>,
    pub table_delete_form: Option<TextForm>,
    pub create_db_form: Option<TextForm>,
    pub should_quit: bool,
    pub options: Options,
    pub key_bindings: KeyBindings,
    pub language: AppLanguage,
    pub perf_profiler: Option<Receiver<Resources>>,
    pub statistics: StatisticsProfiling,
}

impl App {
    pub fn new(
        qualifier: String,
        organization: String,
        application: String,
        default_color_scheme: StaticColors,
    ) -> io::Result<Self> {
        let mut options = Options::load_or_default(
            &qualifier,
            &organization,
            &application,
            default_color_scheme,
        )?;

        options.build_fields();

        let key_bindings = KeyBindings::load_or_default(&qualifier, &organization, &application)?;

        let language = AppLanguage::load_from_file(
            &qualifier,
            &organization,
            &application,
            &options.selected_language,
        )?;

        let file_explorer_strings = (
            language.invalid_utf_8.clone(),
            language.not_available.clone(),
            language.screen_file_explorer_dir.clone(),
            language.bytes.clone(),
            language.app_datetime_format.clone(),
            language.screen_file_explorer_dir_read_error.clone(),
        );

        Ok(Self {
            qualifier,
            organization,
            application,
            current_screen: Screen::Splash,
            current_popup: PopUp::None,
            current_mode: Mode::Browse,
            current_error: None,
            selected_db: None,
            selected_db_table: None,
            selected_table_columns: Vec::new(),
            file_explorer_table: FileExplorerTable::new(file_explorer_strings),
            table_list_view: None,
            column_list_view: None,
            row_list_view: None,
            row_insert_form: None,
            row_delete_form: None,
            table_insert_form: None,
            table_delete_form: None,
            create_db_form: None,
            should_quit: false,
            options,
            key_bindings,
            language,
            perf_profiler: None,
            statistics: StatisticsProfiling::new(10),
        })
    }

    pub fn set_profiler_rx(&mut self, rx: Receiver<Resources>) {
        self.perf_profiler = Some(rx);
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            let start = Instant::now();

            render::render(terminal, self)?;

            let render_duration = start.elapsed();
            self.statistics.push_render_time(render_duration);

            if let Some(rx) = &self.perf_profiler {
                while let Ok(stats) = rx.try_recv() {
                    log(format!("{}", stats).as_str());
                    log(format!("avg frame render time: {:?}", render_duration).as_str());
                    self.statistics.push_cpu_and_memory_values(
                        stats.global_used_cpu,
                        stats.global_used_memory,
                        stats.process_used_cpu,
                        stats.process_used_memory,
                    );
                }
            }

            // handle_key_events poll duration controls the app update rate
            if handle_key_events(self)? {
                break;
            }
        }
        Ok(())
    }

    pub fn open_db_file(&mut self, path: PathBuf) -> Result<(), DBError> {
        let sqlite_extensions = HashSet::from([
            OsString::from("db"),
            OsString::from("db3"),
            OsString::from("s3db"),
            OsString::from("sl3"),
            OsString::from("sqlite"),
            OsString::from("sqlite3"),
        ]);

        if path.is_file() && sqlite_extensions.contains(path.extension().unwrap()) {
            let db_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let db_extension = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            match DB::new(db_name, db_extension) {
                Ok(db) => {
                    self.selected_db = Some(db);
                    self.fetch_table_list();
                    self.populate_table_col_map();
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err(DBError::ConnectionCreationError(
                "Invalid .db file path".to_string(),
            ))
        }
    }

    pub fn fetch_table_list(&mut self) {
        if let Some(db) = &self.selected_db {
            match db.get_table_list() {
                Ok(tables) => {
                    let mut table_info_vec: Vec<TableInfo> = Vec::new();
                    for table_name in &tables {
                        let is_view = db.is_table_view(table_name).unwrap_or(false);
                        let row_count = if !is_view {
                            db.get_table_row_count(table_name).unwrap_or(0)
                        } else {
                            0 // views dont store rows, so we keep this 0, if table is a view
                        };

                        table_info_vec.push(TableInfo {
                            name: table_name.to_string(),
                            row_count,
                            is_view,
                        });
                    }

                    if let Some(first_table) = table_info_vec.first() {
                        self.selected_db_table = Some(first_table.name.to_string());
                        self.selected_table_columns =
                            db.get_table_columns(&first_table.name).unwrap_or_default();
                        self.column_list_view =
                            Some(ColumnListView::new(self.selected_table_columns.clone()));
                        self.row_list_view = None;
                    } else {
                        self.selected_db_table = None;
                        self.column_list_view = None;
                        self.row_list_view = None;
                    }

                    self.table_list_view = Some(TableListView::new(table_info_vec));
                }
                Err(_) => {
                    self.selected_db_table = None;
                    self.table_list_view = None;
                    self.column_list_view = None;
                    self.row_list_view = None;
                }
            }
        } else {
            self.selected_db_table = None;
            self.table_list_view = None;
            self.column_list_view = None;
            self.row_list_view = None;
        }
    }

    pub fn select_table(&mut self, table_name: String) {
        if let Some(db) = &self.selected_db {
            match db.get_table_columns(&table_name) {
                Ok(columns) => {
                    self.selected_db_table = Some(table_name);
                    self.selected_table_columns = columns.clone();
                    self.column_list_view = Some(ColumnListView::new(columns));
                    self.row_list_view = None
                }
                Err(_) => {
                    self.selected_table_columns.clear();
                    self.column_list_view = None;
                    self.row_list_view = None;
                }
            }
        }
    }

    pub fn select_table_rows(&mut self, table_name: String) {
        if let Some(db) = &self.selected_db {
            match db.get_table_rows(&table_name) {
                Ok(rows) => {
                    self.row_list_view = Some(RowListView::new(rows));
                }
                Err(_) => {
                    self.row_list_view = None;
                }
            }
        }
    }

    pub fn open_file(&mut self, path: PathBuf) {
        if let Err(_) = self.open_db_file(path) {
            self.switch_to_popup(PopUp::Error);
            self.current_error = Some(DBError::ConnectionCreationError(
                "Cant open that file".to_string(),
            ))
        } else {
            self.switch_to_screen(Screen::DatabaseSchema);
        }
    }

    pub fn create_row_insert_form(&mut self, table_cols: Vec<String>) {
        let title_text = format!(
            "Enter new entry into table {}",
            self.selected_db_table.as_deref().unwrap()
        );
        self.row_insert_form = Some(TextForm::new(table_cols, title_text));
    }

    pub fn create_row_delete_form(&mut self) {
        let title_text = format!(
            "Delete entry from table {}",
            self.selected_db_table.as_deref().unwrap()
        );
        self.row_delete_form = Some(TextForm::new(
            vec!["Column name".to_string(), "Row value".to_string()],
            title_text,
        ));
    }

    pub fn create_table_insert_form(&mut self) {
        let title_text = format!(
            "Create new table into database {}",
            self.selected_db.as_ref().unwrap().get_db_name()
        );
        self.table_insert_form = Some(TextForm::new(vec!["Raw SQL".to_string()], title_text));
    }

    pub fn create_table_delete_form(&mut self) {
        let title_text = format!(
            "Drop table from database {}",
            self.selected_db.as_ref().unwrap().get_db_name()
        );
        self.table_delete_form = Some(TextForm::new(vec!["Table Name".to_string()], title_text));
    }

    pub fn create_new_db_form(&mut self) {
        let title_text = "Create a new database".to_string();
        self.create_db_form = Some(TextForm::new(vec!["Database name".to_string()], title_text));
    }

    pub fn text_color(&self) -> Color {
        self.options.selected_color_scheme.colors().text
    }

    pub fn text_alt_color(&self) -> Color {
        self.options.selected_color_scheme.colors().text_alt
    }

    pub fn text_highlight_color(&self) -> Color {
        self.options.selected_color_scheme.colors().text_highlight
    }

    pub fn background_color(&self) -> Color {
        self.options.selected_color_scheme.colors().background
    }

    pub fn background_alt_color(&self) -> Color {
        self.options.selected_color_scheme.colors().background_alt
    }

    pub fn background_highlight_color(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .background_highlight
    }

    pub fn warning_color(&self) -> Color {
        self.options.selected_color_scheme.colors().warning
    }

    pub fn error_color(&self) -> Color {
        self.options.selected_color_scheme.colors().error
    }

    pub fn border_color(&self) -> Color {
        self.options.selected_color_scheme.colors().border
    }

    #[allow(dead_code)]
    pub fn accent_color(&self) -> Color {
        self.options.selected_color_scheme.colors().accent
    }

    pub fn switch_to_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn switch_to_popup(&mut self, popup: PopUp) {
        self.current_popup = popup;

        if popup != PopUp::Error {
            self.current_error = None;
        }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.current_mode = mode;
    }

    pub fn show_error(&mut self, error: DBError) {
        self.switch_to_popup(PopUp::Error);
        self.current_error = Some(error);
    }

    pub fn populate_table_col_map(&mut self) {
        if let Some(db) = &mut self.selected_db {
            db.db_tab_col_map.clear();

            if let Ok(tables) = db.get_table_list() {
                for table in tables {
                    if let Ok(cols) = db.get_table_columns(&table) {
                        db.db_tab_col_map
                            .insert(table, cols.iter().map(|col| col.name.clone()).collect());
                    }
                }
            }
        }
    }

    pub fn get_strings_for_col_info(language: &AppLanguage) -> (&str, &str, &str, &str, &str) {
        (
            &language.sql_pk_constraint,
            &language.sql_unique_constraint,
            &language.sql_not_null_constraint,
            &language.sql_fk_constraint,
            &language.column_list_unknown_fk_ref,
        )
    }
}
