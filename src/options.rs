use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    lang::language::SupportedLanguage,
    ui::colors::{
        app_colors::{AppColors, ColorScheme},
        dynamic_colors::DynamicColors,
        static_colors::StaticColors,
    },
    widgets::{generic_list_view::GenericListView, selectable_field::SelectableField},
};

pub enum OptionKind {
    Toggle(bool),
    TextInput(String),
}

#[derive(EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SelectedOption {
    TableMetainfoToggle,
    InsertMetainfoToggle,
    RenderFooter,
    RenderInfoSection,
    InfoSectionHeight,
    LogPerformanceMetrics,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SelectedScheme {
    Static(StaticColors),
    Dynamic(DynamicColors),
}

impl ColorScheme for SelectedScheme {
    fn colors(&self) -> AppColors {
        match self {
            SelectedScheme::Static(s) => s.colors(),
            SelectedScheme::Dynamic(d) => d.colors(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    // colors
    pub available_color_schemes: GenericListView<SelectedScheme>,
    pub selected_color_scheme: SelectedScheme,
    // serialization for selected option
    pub selected_option: SelectedOption,
    pub available_options: Vec<SelectedOption>,
    // available options fields
    #[serde(skip)]
    pub fields: Vec<SelectableField>,
    pub index: usize,
    // language support
    pub available_languages: Vec<SupportedLanguage>,
    pub selected_language: SupportedLanguage,
    // actual options values
    pub display_col_metainfo_in_table_view: bool,
    pub display_col_metainfo_in_insert_view: bool,
    pub render_footer: bool,
    pub render_info_section: bool,
    pub info_section_height: u16,
    pub log_performance_metrics: bool,
}

impl Options {
    pub fn new(default_color_scheme: StaticColors) -> Self {
        let mut schemes = Vec::new();
        schemes.extend(StaticColors::iter().map(SelectedScheme::Static));
        schemes.extend(DynamicColors::iter().map(SelectedScheme::Dynamic));
        let available_color_schemes = GenericListView::new(schemes);
        let available_options = SelectedOption::iter().collect();
        let available_languages = SupportedLanguage::iter().collect();

        Self {
            available_color_schemes,
            selected_color_scheme: SelectedScheme::Static(default_color_scheme),
            selected_option: SelectedOption::TableMetainfoToggle,
            available_options,
            fields: Vec::new(),
            index: 0,
            available_languages,
            selected_language: SupportedLanguage::English,
            display_col_metainfo_in_table_view: true,
            display_col_metainfo_in_insert_view: true,
            render_footer: false,
            render_info_section: true,
            info_section_height: 5,
            log_performance_metrics: false,
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

            let mut schemes = Vec::new();
            schemes.extend(StaticColors::iter().map(SelectedScheme::Static));
            schemes.extend(DynamicColors::iter().map(SelectedScheme::Dynamic));

            options.available_color_schemes = GenericListView::new(schemes);

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

    pub fn build_fields(&mut self) {
        self.fields = vec![
            SelectableField {
                kind: OptionKind::Toggle(self.display_col_metainfo_in_table_view),
                selected: self.selected_option == SelectedOption::TableMetainfoToggle,
            },
            SelectableField {
                kind: OptionKind::Toggle(self.display_col_metainfo_in_insert_view),
                selected: self.selected_option == SelectedOption::InsertMetainfoToggle,
            },
            SelectableField {
                kind: OptionKind::Toggle(self.render_footer),
                selected: self.selected_option == SelectedOption::RenderFooter,
            },
            SelectableField {
                kind: OptionKind::Toggle(self.render_info_section),
                selected: self.selected_option == SelectedOption::RenderInfoSection,
            },
            SelectableField {
                kind: OptionKind::TextInput(self.info_section_height.to_string()),
                selected: self.selected_option == SelectedOption::InfoSectionHeight,
            },
            SelectableField {
                kind: OptionKind::Toggle(self.log_performance_metrics),
                selected: self.selected_option == SelectedOption::LogPerformanceMetrics,
            },
        ]
    }

    pub fn sync_from_fields(&mut self) {
        for (i, field) in self.fields.iter().enumerate() {
            match i {
                0 => {
                    if let OptionKind::Toggle(v) = field.kind {
                        self.display_col_metainfo_in_table_view = v;
                    }
                }
                1 => {
                    if let OptionKind::Toggle(v) = field.kind {
                        self.display_col_metainfo_in_insert_view = v;
                    }
                }
                2 => {
                    if let OptionKind::Toggle(v) = field.kind {
                        self.render_footer = v;
                    }
                }
                3 => {
                    if let OptionKind::Toggle(v) = field.kind {
                        self.render_info_section = v;
                    }
                }
                4 => {
                    if let OptionKind::TextInput(ref s) = field.kind {
                        if let Ok(num) = s.parse::<u16>() {
                            self.info_section_height = num;
                        }
                    }
                }
                5 => {
                    if let OptionKind::Toggle(v) = field.kind {
                        self.log_performance_metrics = v;
                    }
                }
                _ => {}
            }
        }
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

    pub fn previous_option(&mut self) {
        self.fields[self.index].selected = false;

        if self.index == 0 {
            self.index = self.available_options.len() - 1;
        } else {
            self.index -= 1;
        }

        self.selected_option = self.available_options[self.index];
        self.fields[self.index].selected = true;
    }

    pub fn next_option(&mut self) {
        self.fields[self.index].selected = false;
        self.index = (self.index + 1) % self.available_options.len();
        self.selected_option = self.available_options[self.index];
        self.fields[self.index].selected = true;
    }
}
