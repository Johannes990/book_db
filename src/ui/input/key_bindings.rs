use std::{collections::HashMap, fs, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{log::log, ui::input::key_events_serializable::{KeyCodeSerializable, KeyModifierSerializable}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key_code: KeyCodeSerializable,
    pub key_modifier: KeyModifierSerializable,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppInputEvent {
    OpenSplashScreen,           // open initial screen
    OpenFileExplorerScreen,     // open file explorer screen
    OpenDBSchemaScreen,         // open selected database schema screen
    OpenDBTableScreen,          // open selected db table screen
    OpenCreateNewFileScreen,    // open create new db file screen
    OpenOptionsScreen,          // open options screen
    OpenInsertRowPopUp,         // open insert row popup
    OpenDeleteRowPopUp,         // open delete row popup
    OpenInsertTablePopUp,       // open insert new table popup
    OpenDeleteTablePopUp,       // open delete table popup
    QuitApp,                    // open quit app popup
    MoveUpPrimary,              // go up in primary table or in options
    MoveDownPrimary,            // go down in primary table or in options
    MoveUpSecondary,            // go up in secondary table or in colorschemes
    MoveDownSecondary,          // go down in secondary table or in coloschemes
    ExecuteAction,              // execute current popup or SQL action
    ToggleOption,               // toggle selected option on/off
    FileExplorerSelect,         // select folder or file to load
    FileExplorerBack,           // go up to paren folder in file explorer
}

#[derive(Deserialize, Serialize)]
pub struct KeyBindings {
    pub by_event_type: HashMap<AppInputEvent, KeyBinding>,
    #[serde(skip)]
    pub by_event_input: HashMap<KeyBinding, AppInputEvent>,
}

impl KeyBindings {
    pub fn default() -> Self {
        let defaults = Self::get_default_bindings();
        let mut by_event_type = HashMap::new();
        let mut by_event_input = HashMap::new();

        for (event, binding) in defaults.iter() {
            by_event_type.insert(*event, *binding);
            by_event_input.insert(*binding, *event);
        }

        Self { by_event_type, by_event_input }
    }

    pub fn load_or_default(
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
    ) -> io::Result<Self> {
        let project_dirs = ProjectDirs::from(qualifier_str, organization_str, application_str)
            .expect("Could not determine directoy!");
        fs::create_dir_all(project_dirs.config_dir())?;
        let keybindings_dir_path = project_dirs.config_dir().join("keybindings.toml");

        if keybindings_dir_path.exists() {
            let data = fs::read_to_string(&keybindings_dir_path)?;
            let mut keybindings: Self = toml::from_str(&data)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

            keybindings.by_event_input = keybindings.by_event_type
                .iter()
                .map(|(e, b)| (*b, *e))
                .collect();

            Ok(keybindings)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(
        &self,
        qualifier_str: &str,
        organization_str: &str,
        application_str: &str,
    ) -> io::Result<()> {
        log("Entering save keybindings function");
        let project_dirs = ProjectDirs::from(qualifier_str, organization_str, application_str)
            .expect("Could not determine directory!");
        log(format!("project dirs: {:?}", project_dirs).as_str());
        let keybindings_path = project_dirs.config_dir().join("keybindings.toml");
        log(format!("keybindings path: {:?}", keybindings_path).as_str());
        fs::create_dir_all(project_dirs.config_dir())?;
        log("created keybindings path");
        let data =
            toml::to_string(self).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
            log("toml parsed");
        fs::write(keybindings_path, data)?;
        log("filesystem write done");
        Ok(())
    }

    pub fn resolve_event(&self, key_event: KeyEvent) -> Option<AppInputEvent> {
        let binding = KeyBinding {
            key_code: key_event.code.into(),
            key_modifier: key_event.modifiers.into(),
        };
        self.by_event_input.get(&binding).copied()
    }

    fn get_default_bindings() -> Vec<(AppInputEvent, KeyBinding)> {
        vec![
            (AppInputEvent::OpenSplashScreen, KeyBinding { key_code: KeyCode::Char('s').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenFileExplorerScreen, KeyBinding { key_code: KeyCode::Char('f').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenDBSchemaScreen, KeyBinding { key_code: KeyCode::Char('d').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenDBTableScreen, KeyBinding { key_code: KeyCode::Char('t').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenCreateNewFileScreen, KeyBinding { key_code: KeyCode::Char('c').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenOptionsScreen, KeyBinding { key_code: KeyCode::Char('o').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenInsertRowPopUp, KeyBinding { key_code: KeyCode::Char('i').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenDeleteRowPopUp, KeyBinding { key_code: KeyCode::Char('d').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenInsertTablePopUp, KeyBinding { key_code: KeyCode::Char('i').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::OpenDeleteTablePopUp, KeyBinding { key_code: KeyCode::Char('d').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::QuitApp, KeyBinding { key_code: KeyCode::Char('q').into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::MoveUpPrimary, KeyBinding { key_code: KeyCode::Up.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::MoveDownPrimary, KeyBinding { key_code: KeyCode::Down.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::MoveUpSecondary, KeyBinding { key_code: KeyCode::Left.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::MoveDownSecondary, KeyBinding { key_code: KeyCode::Right.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::ExecuteAction, KeyBinding { key_code: KeyCode::Enter.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::ToggleOption, KeyBinding { key_code: KeyCode::Tab.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::FileExplorerSelect, KeyBinding { key_code: KeyCode::Enter.into(), key_modifier: KeyModifiers::NONE.into() }),
            (AppInputEvent::FileExplorerBack, KeyBinding { key_code: KeyCode::Backspace.into(), key_modifier: KeyModifiers::NONE.into() }),
        ]
    }
}
