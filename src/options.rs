use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    ui::colors::{
        app_colors::{AppColors, ColorScheme},
        static_colors::StaticColors,
    },
    widgets::generic_list_view::GenericListView,
};

#[derive(EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SelectedOption {
    TableMetainfoToggle,
    InsertMetainfoToggle,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SelectedScheme {
    Static(StaticColors),
}

impl ColorScheme for SelectedScheme {
    fn colors(&self) -> AppColors {
        match self {
            SelectedScheme::Static(s) => s.colors(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub available_color_schemes: GenericListView<SelectedScheme>,
    pub selected_color_scheme: SelectedScheme,
    pub available_options: Vec<SelectedOption>,
    pub selected_option: SelectedOption,
    pub display_col_metainfo_in_table_view: bool,
    pub display_col_metainfo_in_insert_view: bool,
}

impl Options {
    pub fn new(default_color_scheme: StaticColors) -> Self {
        let available_color_schemes = GenericListView::new(
            StaticColors::iter()
                .map(SelectedScheme::Static)
                .collect::<Vec<_>>()
        );
        let available_options = SelectedOption::iter().collect();
        Self {
            available_color_schemes,
            selected_color_scheme: SelectedScheme::Static(default_color_scheme),
            available_options,
            selected_option: SelectedOption::TableMetainfoToggle,
            display_col_metainfo_in_table_view: true,
            display_col_metainfo_in_insert_view: true,
        }
    }

    pub fn load_or_default(
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
        default_color_scheme: StaticColors,
    ) -> io::Result<Self> {
        let project_dirs = ProjectDirs::from(qualifier_str, organization_str, application_str)
            .expect("Could not determine directoy!");
        fs::create_dir_all(project_dirs.config_dir())?;
        let config_dir_path = project_dirs.config_dir().join("config.toml");

        if config_dir_path.exists() {
            let data = fs::read_to_string(&config_dir_path)?;
            let mut options: Self = toml::from_str(&data)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

            options.available_color_schemes.rebuild();

            options.available_color_schemes =
                GenericListView::new(StaticColors::iter().map(SelectedScheme::Static).collect());

            if !options
                .available_color_schemes
                .items
                .contains(&options.selected_color_scheme)
            {
                options.selected_color_scheme = SelectedScheme::Static(default_color_scheme);
            }

            Ok(options)
        } else {
            Ok(Self::new(default_color_scheme))
        }
    }

    pub fn save(
        &self,
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
    ) -> io::Result<()> {
        let project_dirs = ProjectDirs::from(qualifier_str, organization_str, application_str)
            .expect("Could not determine directory!");
        let config_path = project_dirs.config_dir().join("config.toml");
        fs::create_dir_all(project_dirs.config_dir())?;
        let data =
            toml::to_string(self).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        fs::write(config_path, data)?;
        Ok(())
    }

    fn select_color_scheme(&mut self, color_scheme: SelectedScheme) {
        if self.available_color_schemes.items.contains(&color_scheme) {
            self.selected_color_scheme = color_scheme;
        }
    }

    pub fn previous_color_scheme(&mut self) {
        self.available_color_schemes.previous();
        let index = self.available_color_schemes.index;
        self.select_color_scheme(self.available_color_schemes.items[index]);
    }

    pub fn next_color_scheme(&mut self) {
        self.available_color_schemes.next();
        let index = self.available_color_schemes.index;
        self.select_color_scheme(self.available_color_schemes.items[index]);
    }

    pub fn set_display_col_metainfo_in_table_view(&mut self, value: bool) {
        self.display_col_metainfo_in_table_view = value;
    }

    pub fn set_display_col_metainfo_in_insert_view(&mut self, value: bool) {
        self.display_col_metainfo_in_insert_view = value;
    }

    pub fn previous_option(&mut self) {
        if let Some(index) = self
            .available_options
            .iter()
            .position(|&so| so == self.selected_option)
        {
            let prev_index = if index == 0 {
                self.available_options.len() - 1
            } else {
                index - 1
            };
            self.select_option(self.available_options[prev_index]);
        }
    }

    pub fn next_option(&mut self) {
        if let Some(index) = self
            .available_options
            .iter()
            .position(|&so| so == self.selected_option)
        {
            let next_index = (index + 1) % self.available_options.len();
            self.select_option(self.available_options[next_index]);
        }
    }

    fn select_option(&mut self, option: SelectedOption) {
        if self.available_options.contains(&option) {
            self.selected_option = option;
        }
    }
}
