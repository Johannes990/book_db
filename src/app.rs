use crate::{
    column::{column_info::ColumnInfo, column_list::ColumnListView},
    db::{DBError, DB},
    file_explorer::file_explorer_table::FileExplorerTable,
    handle_key_events,
    options::Options,
    row::row_list::RowListView,
    table::{table_info::TableInfo, table_list::TableListView},
    ui::{colorscheme::ColorScheme, render}, widgets::{table_delete_form::TableDeleteForm, table_insert_form::TableInsertForm},
};
use ratatui::{
    style::Color,
    Terminal,
};
use std::{io, path::PathBuf};

pub enum Screen {
    Splash,
    FileExplorer,
    DatabaseSchema,
    DataBaseTable,
    CreateNewFile,
    Options,
}

pub enum PopUp {
    None,
    Quit,
    NoDBLoaded,
    InsertRow,
    DeleteRow
}

pub enum AppMode {
    Browsing,
    Editing,
}

pub struct App {
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub current_mode: AppMode,
    pub selected_db: Option<DB>,
    pub selected_db_table: Option<String>,
    pub selected_table_columns: Vec<ColumnInfo>,
    pub file_explorer_table: FileExplorerTable,
    pub table_list_view: Option<TableListView>,
    pub column_list_view: Option<ColumnListView>,
    pub row_list_view: Option<RowListView>,
    pub table_insert_form: Option<TableInsertForm>,
    pub table_delete_form: Option<TableDeleteForm>,
    pub should_quit: bool,
    pub options: Options
}

impl App {
    pub fn new(color_scheme: ColorScheme) -> Self {
        let file_explorer_table = FileExplorerTable::new();
        let options = Options::new(color_scheme);

        Self {
            current_screen: Screen::Splash,
            current_popup: PopUp::None,
            current_mode: AppMode::Browsing,
            selected_db: None,
            selected_db_table: None,
            selected_table_columns: Vec::new(),
            file_explorer_table,
            table_list_view: None,
            column_list_view: None,
            row_list_view: None,
            table_insert_form: None,
            table_delete_form: None,
            should_quit: false,
            options
        }
    }

    pub fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            render::render(terminal, self)?;
    
            if handle_key_events(self)? {
                break;
            }
        }
        Ok(())
    }

    pub fn open_db_file(&mut self, path: PathBuf) -> Result<(), DBError> {
        if path.is_file() && path.extension().unwrap() == "db" {
            let db_name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

            match DB::new(db_name) {
                Ok(db) => {
                    self.selected_db = Some(db);
                    self.fetch_table_list();
                    Ok(())
                },
                Err(e) => Err(e),
            }
        } else {
            Err(DBError::ConnectionCreationError("Invalid .db file path".to_string()))
        }
    }

    fn fetch_table_list(&mut self) {
        if let Some(db) = &self.selected_db {
            match db.get_table_list() {
                Ok(tables) => {
                    let mut table_info_vec: Vec<TableInfo> = Vec::new();
                    for table_name in &tables {
                        let is_view = db.is_table_view(&table_name).unwrap_or(false);
                        let row_count = if !is_view {
                            db.get_table_row_count(&table_name).unwrap_or(0)
                        } else {
                            0 // views dont store rows, so we keep this 0, if table is a view
                        };

                        table_info_vec.push(TableInfo {
                            name: table_name.to_string(),
                            row_count,
                            is_view 
                        });
                    }

                    if let Some(first_table) = table_info_vec.first() {
                        self.selected_db_table = Some(first_table.name.to_string());
                        self.selected_table_columns = db.get_table_columns(&first_table.name).unwrap_or_default();
                        self.column_list_view = Some(ColumnListView::new(self.selected_table_columns.clone()));
                        self.row_list_view = None;
                    } else {
                        self.selected_db_table = None;
                        self.column_list_view = None;
                        self.row_list_view = None;
                    }

                    self.table_list_view = Some(TableListView::new(table_info_vec));
                },
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

    pub fn create_table_insert_form(&mut self, table_cols: Vec<String>) {
        self.table_insert_form = Some(TableInsertForm::new(table_cols));
    }

    pub fn create_table_delete_form(&mut self) {
        self.table_delete_form = Some(TableDeleteForm::new("", ""));
    }

    pub fn general_text_color(&self) -> Color {
        self.options.selected_color_scheme.colors().general_text_color
    }

    pub fn alt_text_color_1(&self) -> Color {
        self.options.selected_color_scheme.colors().alt_text_color_1
    }

    pub fn alt_text_color_2(&self) -> Color {
        self.options.selected_color_scheme.colors().alt_text_color_2
    }

    pub fn general_page_bg_color(&self) -> Color {
        self.options.selected_color_scheme.colors().general_page_bg_color
    }

    pub fn quit_popup_bg_col(&self) -> Color {
        self.options.selected_color_scheme.colors().quit_popup_bg_col
    }

    pub fn file_exp_pg_selected_col(&self) -> Color {
        self.options.selected_color_scheme.colors().file_exp_pg_selected_col
    }

    pub fn table_row_normal_col(&self) -> Color {
        self.options.selected_color_scheme.colors().table_row_normal_col
    }

    pub fn table_row_alt_color(&self) -> Color {
        self.options.selected_color_scheme.colors().table_row_alt_color
    }

    pub fn info_block_bg_col(&self) -> Color {
        self.options.selected_color_scheme.colors().info_block_bg_col
    }

    pub fn text_entry_box_bg_col(&self) -> Color {
        self.options.selected_color_scheme.colors().text_entry_box_bg_col
    }

    pub fn switch_to_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn switch_to_popup(&mut self, popup: PopUp) {
        self.current_popup = popup;
    }

    pub fn list_available_color_schemes(&self) -> &Vec<ColorScheme> {
        self.options.list_color_schemes()
    }

    pub fn switch_app_mode(&mut self, mode: AppMode) {
        self.current_mode = mode;
    }
}
