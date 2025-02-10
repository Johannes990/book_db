use crate::{
    column::{column_info::ColumnInfo, column_list::ColumnListView},
    db::{DBError, DB},
    fex::fex_table::FileExplorerTable,
    handle_key_events,
    options::Options,
    table::table_list::TableListView,
    ui::{colorscheme::ColorScheme, render}
};
use ratatui::{
    style::Color,
    Terminal,
};
use std::{io, path::PathBuf};

pub enum Screen {
    SplashScreenView,
    FileExplorerView,
    OpenDataBaseView,
    CreateNewFileView,
    OptionsView,
}

pub enum PopUp {
    None,
    QuitDialog,
    NoDBLoadedDialog,
    SaveDialog,
}

pub struct App {
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub selected_db: Option<DB>,
    pub selected_db_table: Option<String>,
    pub selected_table_columns: Vec<ColumnInfo>,
    pub file_explorer_table: FileExplorerTable,
    pub table_list_view: Option<TableListView>,
    pub column_list_view: Option<ColumnListView>,
    pub options: Options
}

impl App {
    pub fn new(color_scheme: ColorScheme) -> Self {
        let file_explorer_table = FileExplorerTable::new();
        let options = Options::new(color_scheme);

        Self {
            current_screen: Screen::SplashScreenView,
            current_popup: PopUp::None,
            selected_db: None,
            selected_db_table: None,
            selected_table_columns: Vec::new(),
            file_explorer_table,
            table_list_view: None,
            column_list_view: None,
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
                    match db.get_table_list() {
                        Ok(tables) => {
                            if let Some(first_table) = &tables.first() {
                                self.selected_db_table = Some(first_table.to_string());
                                self.table_list_view = Some(TableListView::new(tables.clone()));

                                match db.get_table_columns(first_table) {
                                    Ok(columns) => {
                                        self.selected_table_columns = columns.clone();
                                        self.column_list_view = Some(ColumnListView::new(columns));
                                    },
                                    Err(_) => {
                                        self.selected_table_columns.clear();
                                        self.column_list_view = None;
                                    }
                                }
                            } else {
                                self.selected_db_table = None;
                                self.table_list_view = None;
                                self.column_list_view = None;
                            }
                        },
                        Err(_) => {
                            self.selected_db_table = None;
                            self.table_list_view = None;
                            self.column_list_view = None;
                        }
                    }
                    self.selected_db = Some(db);
                    Ok(())
                },
                Err(e) => {
                    Err(e)
                }
            }
        } else {
            Err(DBError::ConnectionCreationError("Invalid .db file path".to_string()))
        }
    }

    pub fn get_db(&mut self) -> &mut DB {
        Option::expect(self.selected_db.as_mut(), "No db loaded")
    }

    pub fn select_table(&mut self, table_name: String) {
        if let Some(db) = &self.selected_db {
            match db.get_table_columns(&table_name) {
                Ok(columns) => {
                    self.selected_db_table = Some(table_name);
                    self.selected_table_columns = columns.clone();
                    self.column_list_view = Some(ColumnListView::new(columns));
                }
                Err(_) => {
                    self.selected_table_columns.clear();
                    self.column_list_view = None;
                }
            }
        }
    }

    pub fn update_column_list(&mut self) {
        if let Some(db) = &self.selected_db {
            if let Some(table_list_view) = &self.table_list_view {
                if let Some(selected_table) = table_list_view.items.get(table_list_view.index) {
                    self.column_list_view = Some(ColumnListView::new(db.get_table_columns(selected_table).unwrap()));
                }
            }
        }
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

    pub fn switch_to_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn switch_to_popup(&mut self, popup: PopUp) {
        self.current_popup = popup;
    }

    pub fn update_color_scheme(&mut self, new_color_scheme: ColorScheme) {
        self.options.select_color_scheme(new_color_scheme);
    }

    pub fn list_available_color_schemes(&self) -> &Vec<ColorScheme> {
        self.options.list_color_schemes()
    }
}
