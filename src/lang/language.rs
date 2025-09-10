use directories_next::ProjectDirs;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Deserialize, Eq, EnumIter, Hash, PartialEq, Serialize)]
pub enum SupportedLanguage {
    English,
}

impl SupportedLanguage {
    pub fn file_path(&self) -> &'static str {
        match self {
            SupportedLanguage::English => "en.toml",
        }
    }
}

#[derive(Embed)]
#[folder = "lang/"]
struct LanguageAsset;

#[derive(Debug, Clone, Deserialize)]
pub struct AppLanguage {
    pub event_open_splash_screen: String,
    pub event_open_file_explorer_screen: String,
    pub event_open_db_schema_screen: String,
    pub event_open_db_table_screen: String,
    pub event_open_create_new_file_screen: String,
    pub event_open_options_screen: String,
    pub event_open_insert_row_popup: String,
    pub event_open_delete_row_popup: String,
    pub event_open_insert_table_popup: String,
    pub event_open_delete_table_popup: String,
    pub event_close_popup: String,
    pub event_open_quit_app_popup: String,
    pub event_quit_app_confirm: String,
    pub event_move_up_primary: String,
    pub event_move_down_primary: String,
    pub event_move_up_secondary: String,
    pub event_move_down_secondary: String,
    pub event_execute_action: String,
    pub event_toggle_option: String,
    pub event_file_explorer_select: String,
    pub event_file_explorer_back: String,
    pub event_switch_to_edit: String,
    pub event_switch_to_browse: String,
    pub screen_splash_db_placeholder: String,
    pub screen_splash_table_placeholder: String,
    pub screen_splash_application_name: String,
    pub screen_splash_loaded_db: String,
    pub screen_splash_loaded_table: String,
    pub screen_file_explorer_title: String,
    pub screen_file_explorer_current_location: String,
    pub screen_file_explorer_file_folder_header: String,
    pub screen_file_explorer_size_header: String,
    pub screen_file_explorer_date_created_header: String,
    pub screen_file_explorer_dir: String,
    pub screen_file_explorer_dir_read_error: String,
    pub screen_db_schema_no_db_found: String,
    pub screen_db_schema_current_db: String,
    pub screen_db_table_current_table: String,
    pub screen_db_table_table_placeholder: String,
    pub screen_options_title: String,
    pub screen_options_color_schemes: String,
    pub screen_options_metadata_in_table: String,
    pub screen_options_metadata_in_insert: String,
    pub screen_options_render_info: String,
    pub screen_options_info_block_height: String,
    pub popup_quit_confirmation: String,
    pub popup_no_db_loaded: String,
    pub popup_error_title: String,
    pub table_list_title: String,
    pub table_list_emtpy_placeholder: String,
    pub table_list_name_header: String,
    pub table_list_rows_header: String,
    pub table_list_type_header: String,
    pub table_list_view_element: String,
    pub table_list_table_element: String,
    pub column_list_title: String,
    pub column_list_emtpy_placeholder: String,
    pub column_list_name_header: String,
    pub column_list_type_header: String,
    pub column_list_constraints_header: String,
    pub column_list_unknown_fk_ref: String,
    pub info_block_title: String,

    pub mode_current_mode: String,

    pub widget_selectable_field_highlight_marker: String,
    pub widget_selectable_field_on_value: String,
    pub widget_selectable_field_off_value: String,

    pub sql_pk_constraint: String,
    pub sql_unique_constraint: String,
    pub sql_not_null_constraint: String,
    pub sql_fk_constraint: String,

    pub invalid_utf_8: String,
    pub not_available: String,
    pub bytes: String,
    pub app_datetime_format: String,
}

impl AppLanguage {
    pub fn load_from_file(
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
        selected_lang: &SupportedLanguage,
    ) -> io::Result<Self> {
        let lang_dir = copy_lang_files(qualifier_str, organization_str, application_str)?;
        let file_path = lang_dir.join(selected_lang.file_path());
        let data = fs::read_to_string(file_path)?;
        toml::from_str(&data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}

fn copy_lang_files(
    qualifier_str: &str,
    organization_str: &str,
    application_str: &str,
) -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from(qualifier_str, organization_str, application_str)
        .expect("Could not determine directoy!");
    let lang_dir = project_dirs.config_dir().join("lang");

    fs::create_dir_all(&lang_dir)?;

    for file in LanguageAsset::iter() {
        let target = lang_dir.join(&*file);
        if !target.exists() {
            if let Some(content) = LanguageAsset::get(&file) {
                fs::write(&target, content.data)?;
            }
        }
    }

    Ok(lang_dir)
}
