use crate::{
    column::{column_info::ColumnInfo, column_list::ColumnListView},
    db::{DBError, DB},
    file_explorer::file_explorer_table::FileExplorerTable,
    handle_key_events,
    options::Options,
    row::row_list::RowListView,
    table::{
        table_info::TableInfo,
        table_list::TableListView
    },
    ui::{colorscheme::ColorScheme, render},
    widgets::text_form::TextForm,
};
use ratatui::{style::Color, Terminal};
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
    DeleteRow,
    InsertTable,
    DeleteTable,
    Error(DBError),
}

pub struct App {
    pub qualifier: String,
    pub organization: String,
    pub application: String,
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub selected_db: Option<DB>,
    pub selected_db_table: Option<String>,
    pub selected_table_columns: Vec<ColumnInfo>,
    pub file_explorer_table: FileExplorerTable,
    pub table_list_view: Option<TableListView>,
    pub column_list_view: Option<ColumnListView>,
    pub row_list_view: Option<RowListView>,
    pub table_insert_form: Option<TextForm>,
    pub table_delete_form: Option<TextForm>,
    pub create_table_form: Option<TextForm>,
    pub drop_table_form: Option<TextForm>,
    pub create_db_form: Option<TextForm>,
    pub should_quit: bool,
    pub options: Options,
}

impl App {
    pub fn new(
        qualifier: String,
        organization: String,
        application: String,
        default_color_scheme: ColorScheme,
    ) -> io::Result<Self> {
        let options = Options::load_or_default(
            &qualifier,
            &organization,
            &application,
            default_color_scheme,
        )?;

        Ok(Self {
            qualifier,
            organization,
            application,
            current_screen: Screen::Splash,
            current_popup: PopUp::None,
            selected_db: None,
            selected_db_table: None,
            selected_table_columns: Vec::new(),
            file_explorer_table: FileExplorerTable::new(),
            table_list_view: None,
            column_list_view: None,
            row_list_view: None,
            table_insert_form: None,
            table_delete_form: None,
            create_table_form: None,
            drop_table_form: None,
            create_db_form: None,
            should_quit: false,
            options,
        })
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
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
            let db_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            match DB::new(db_name) {
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

    pub fn create_table_insert_form(&mut self, table_cols: Vec<String>) {
        let title_text = format!(
            "Enter new entry into table {}",
            self.selected_db_table.as_deref().unwrap()
        );
        self.table_insert_form = Some(TextForm::new(table_cols, title_text));
    }

    pub fn create_table_delete_form(&mut self) {
        let title_text = format!(
            "Delete entry from table {}",
            self.selected_db_table.as_deref().unwrap()
        );
        self.table_delete_form = Some(TextForm::new(
            vec!["Column name".to_string(), "Row value".to_string()],
            title_text
        ));
    }

    pub fn create_create_table_form(&mut self) {
        let title_text = format!(
            "Create new table into database {}",
            self.selected_db.as_ref().unwrap().get_db_name()
        );
        self.create_table_form = Some(TextForm::new(vec!["Raw SQL".to_string()], title_text));
    }

    pub fn create_drop_table_form(&mut self) {
        let title_text = format!(
            "Drop table from database {}",
            self.selected_db.as_ref().unwrap().get_db_name()
        );
        self.drop_table_form = Some(TextForm::new(vec!["Table Name".to_string()], title_text));
    }

    pub fn create_new_db_form(&mut self) {
        let title_text = "Create a new database".to_string();
        self.create_db_form = Some(TextForm::new(vec!["Database name".to_string()], title_text));
    }

    pub fn general_text_color(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .general_text_color
    }

    pub fn alt_text_color_1(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .alt_text_color_1
    }

    pub fn alt_text_color_2(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .alt_text_color_2
    }

    pub fn general_page_bg_color(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .general_page_bg_color
    }

    pub fn quit_popup_bg_col(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .quit_popup_bg_col
    }

    pub fn file_exp_pg_selected_col(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .file_exp_pg_selected_col
    }

    pub fn table_row_normal_col(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .table_row_normal_col
    }

    pub fn table_row_alt_color(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .table_row_alt_color
    }

    pub fn info_block_bg_col(&self) -> Color {
        self.options.selected_color_scheme.colors().info_block_bg_col
    }

    pub fn text_entry_box_bg_col(&self) -> Color {
        self.options
            .selected_color_scheme
            .colors()
            .text_entry_box_bg_col
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
}
