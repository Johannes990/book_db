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
    pub open_splash_screen: String,
    pub open_file_explorer_screen: String,
    pub open_db_schema_screen: String,
    pub open_db_table_screen: String,
    pub open_create_new_file_screen: String,
    pub open_options_screen: String,
    pub open_insert_row_popup: String,
    pub open_delete_row_popup: String,
    pub open_insert_table_popup: String,
    pub open_delete_table_popup: String,
    pub close_popup: String,
    pub open_quit_app_popup: String,
    pub quit_app_confirm: String,
    pub move_up_primary: String,
    pub move_down_primary: String,
    pub move_up_secondary: String,
    pub move_down_secondary: String,
    pub execute_action: String,
    pub toggle_option: String,
    pub file_explorer_select: String,
    pub file_explorer_back: String,
    pub switch_to_edit: String,
    pub switch_to_browse: String,
}

impl AppLanguage {
    pub fn load_from_file(
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
        selected_lang: &SupportedLanguage
    ) -> io::Result<Self> {
        let lang_dir = copy_lang_files(qualifier_str, organization_str, application_str)?;
        let file_path = lang_dir.join(selected_lang.file_path());
        let data = fs::read_to_string(file_path)?;
        toml::from_str(&data)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
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
