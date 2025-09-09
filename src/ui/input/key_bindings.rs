use core::fmt;
use std::{collections::HashMap, fs, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{
    app::{Mode, PopUp, Screen},
    lang::language::AppLanguage,
    log::log,
    ui::input::{
        input_context::{context_event, get_input_contexts, InputContext},
        key_events_serializable::{KeyCodeSerializable, KeyModifierSerializable},
    },
};

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppInputEvent {
    OpenSplashScreen,        // open initial screen
    OpenFileExplorerScreen,  // open file explorer screen
    OpenDBSchemaScreen,      // open selected database schema screen
    OpenDBTableScreen,       // open selected db table screen
    OpenCreateNewFileScreen, // open create new db file screen
    OpenOptionsScreen,       // open options screen
    OpenInsertRowPopUp,      // open insert row popup
    OpenDeleteRowPopUp,      // open delete row popup
    OpenInsertTablePopUp,    // open insert new table popup
    OpenDeleteTablePopUp,    // open delete table popup
    ClosePopUp,              // close popup meaning switch to PopUp::None
    OpenQuitAppPopUp,        // open quit app popup
    QuitAppConfirm,          // confirm quit while in quit app
    MoveUpPrimary,           // go up in primary table or in options
    MoveDownPrimary,         // go down in primary table or in options
    MoveUpSecondary,         // go up in secondary table or in colorschemes
    MoveDownSecondary,       // go down in secondary table or in coloschemes
    ExecuteAction,           // execute current popup or SQL action
    ToggleOption,            // toggle selected option on/off
    FileExplorerSelect,      // select folder or file to load
    FileExplorerBack,        // go up to parent folder in file explorer
    SwitchToEdit,            // switch to app edit mode, allowing text editing in text fields
    SwitchToBrowse, // switch to app browse mode, allowing use of commands via simple keystrokes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key_code: KeyCodeSerializable,
    pub key_modifier: KeyModifierSerializable,
    // might use a set of InpuContexts later on if we need to check several
    pub context: InputContext,
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.key_modifier != KeyModifierSerializable::None {
            write!(f, "{} + {}", self.key_code, self.key_modifier)
        } else {
            write!(f, "{}", self.key_code)
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KeyBindingEntry {
    pub context: InputContext,
    pub event: AppInputEvent,
    pub binding: KeyBinding,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyBindingsSerializable {
    pub bindings: Vec<KeyBindingEntry>,
}

#[derive(Deserialize)]
pub struct KeyBindings {
    pub config: KeyBindingsSerializable,
    #[serde(skip)]
    pub by_key_binding: HashMap<(InputContext, KeyBinding), AppInputEvent>,
    #[serde(skip)]
    pub by_app_event: HashMap<AppInputEvent, (InputContext, KeyBinding)>,
}

impl KeyBindings {
    pub fn default() -> Self {
        let defaults: Vec<((InputContext, AppInputEvent), KeyBinding)> =
            Self::get_default_bindings();
        let mut bindings = Vec::new();
        let mut by_key_binding = HashMap::new();
        let mut by_app_event = HashMap::new();

        for ((context, event), binding) in defaults {
            bindings.push(KeyBindingEntry {
                context,
                event,
                binding,
            });
            by_key_binding.insert((context, binding), event);
            by_app_event.insert(event, (context, binding));
        }

        Self {
            config: KeyBindingsSerializable { bindings },
            by_key_binding,
            by_app_event,
        }
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
            let config: KeyBindingsSerializable = toml::from_str(&data)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

            let by_key_binding = config
                .bindings
                .iter()
                .map(|entry| ((entry.context, entry.binding), entry.event))
                .collect();

            let by_app_event = config
                .bindings
                .iter()
                .map(|entry| (entry.event, (entry.context, entry.binding)))
                .collect();

            Ok(Self {
                config,
                by_key_binding,
                by_app_event,
            })
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

        let data = toml::to_string(&self.config).map_err(|err| {
            let error_msg = format!("{}", err);
            log(&error_msg);
            io::Error::new(io::ErrorKind::InvalidData, err)
        })?;
        log("toml parsed");
        fs::write(keybindings_path, data)?;
        log("filesystem write done");
        Ok(())
    }

    pub fn resolve_event(
        &self,
        current_screen: Screen,
        current_popup: PopUp,
        current_mode: Mode,
        key_event: KeyEvent,
    ) -> Option<AppInputEvent> {
        let log_msg = format!(
            "resolving input event: {:?}, current popup: {:?}, current screen: {:?}, current mode: {:?}",
            key_event, current_popup, current_screen, current_mode
        );
        log(&log_msg);

        let binding = KeyBinding {
            key_code: key_event.code.into(),
            key_modifier: key_event.modifiers.into(),
            context: InputContext::Global,
        };

        let contexts = get_input_contexts(current_screen, current_popup, current_mode);

        for context in contexts {
            let binding_with_context = KeyBinding { context, ..binding };

            if let Some(event) = self.by_key_binding.get(&(context, binding_with_context)) {
                log(&format!("Found event {:?} in context {:?}", event, context));
                return Some(*event);
            }
        }

        log("No matching event found in any context");
        None
    }

    pub fn get_info_bits_from_events(
        &self,
        events: &[AppInputEvent],
        language: &AppLanguage,
    ) -> Vec<String> {
        let mut info_bits = Vec::new();

        for event in events {
            let key_str = self
                .by_app_event
                .get(event)
                .map(|(_context, binding)| binding.to_string())
                .unwrap_or_else(|| "?".to_string());

            let desc = match event {
                AppInputEvent::OpenSplashScreen => &language.event_open_splash_screen,
                AppInputEvent::OpenFileExplorerScreen => &language.event_open_file_explorer_screen,
                AppInputEvent::OpenDBSchemaScreen => &language.event_open_db_schema_screen,
                AppInputEvent::OpenDBTableScreen => &language.event_open_db_table_screen,
                AppInputEvent::OpenCreateNewFileScreen => {
                    &language.event_open_create_new_file_screen
                }
                AppInputEvent::OpenOptionsScreen => &language.event_open_options_screen,
                AppInputEvent::OpenInsertRowPopUp => &language.event_open_insert_row_popup,
                AppInputEvent::OpenDeleteRowPopUp => &language.event_open_delete_row_popup,
                AppInputEvent::OpenInsertTablePopUp => &language.event_open_insert_table_popup,
                AppInputEvent::OpenDeleteTablePopUp => &language.event_open_delete_table_popup,
                AppInputEvent::ClosePopUp => &language.event_close_popup,
                AppInputEvent::OpenQuitAppPopUp => &language.event_open_quit_app_popup,
                AppInputEvent::QuitAppConfirm => &language.event_quit_app_confirm,
                AppInputEvent::MoveUpPrimary => &language.event_move_up_primary,
                AppInputEvent::MoveDownPrimary => &language.event_move_down_primary,
                AppInputEvent::MoveUpSecondary => &language.event_move_up_secondary,
                AppInputEvent::MoveDownSecondary => &language.event_move_down_secondary,
                AppInputEvent::ExecuteAction => &language.event_execute_action,
                AppInputEvent::ToggleOption => &language.event_toggle_option,
                AppInputEvent::FileExplorerSelect => &language.event_file_explorer_select,
                AppInputEvent::FileExplorerBack => &language.event_file_explorer_back,
                AppInputEvent::SwitchToEdit => &language.event_switch_to_edit,
                AppInputEvent::SwitchToBrowse => &language.event_switch_to_browse,
            };

            info_bits.push(key_str);
            info_bits.push(desc.clone());
        }

        info_bits
    }

    fn get_default_bindings() -> Vec<((InputContext, AppInputEvent), KeyBinding)> {
        vec![
            context_event(
                KeyCode::Char('s'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenSplashScreen,
            ),
            context_event(
                KeyCode::Char('f'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenFileExplorerScreen,
            ),
            context_event(
                KeyCode::Char('d'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenDBSchemaScreen,
            ),
            context_event(
                KeyCode::Char('t'),
                KeyModifiers::NONE,
                InputContext::Screen(Screen::DatabaseSchema),
                AppInputEvent::OpenDBTableScreen,
            ),
            context_event(
                KeyCode::Char('c'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenCreateNewFileScreen,
            ),
            context_event(
                KeyCode::Char('o'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenOptionsScreen,
            ),
            context_event(
                KeyCode::Char('i'),
                KeyModifiers::NONE,
                InputContext::Screen(Screen::DataBaseTable),
                AppInputEvent::OpenInsertRowPopUp,
            ),
            context_event(
                KeyCode::Char('d'),
                KeyModifiers::NONE,
                InputContext::Screen(Screen::DataBaseTable),
                AppInputEvent::OpenDeleteRowPopUp,
            ),
            context_event(
                KeyCode::Char('i'),
                KeyModifiers::NONE,
                InputContext::Screen(Screen::DatabaseSchema),
                AppInputEvent::OpenInsertTablePopUp,
            ),
            context_event(
                KeyCode::Char('d'),
                KeyModifiers::NONE,
                InputContext::Screen(Screen::DatabaseSchema),
                AppInputEvent::OpenDeleteTablePopUp,
            ),
            context_event(
                KeyCode::Esc,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::ClosePopUp,
            ),
            context_event(
                KeyCode::Char('q'),
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::OpenQuitAppPopUp,
            ),
            context_event(
                KeyCode::Char('y'),
                KeyModifiers::NONE,
                InputContext::PopUp(PopUp::Quit),
                AppInputEvent::QuitAppConfirm,
            ),
            context_event(
                KeyCode::Up,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::MoveUpPrimary,
            ),
            context_event(
                KeyCode::Down,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::MoveDownPrimary,
            ),
            context_event(
                KeyCode::Left,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::MoveUpSecondary,
            ),
            context_event(
                KeyCode::Right,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::MoveDownSecondary,
            ),
            context_event(
                KeyCode::Enter,
                KeyModifiers::NONE,
                InputContext::Global,
                AppInputEvent::ExecuteAction,
            ),
            context_event(
                KeyCode::Tab,
                KeyModifiers::NONE,
                InputContext::Screen(Screen::Options),
                AppInputEvent::ToggleOption,
            ),
            context_event(
                KeyCode::Enter,
                KeyModifiers::NONE,
                InputContext::Screen(Screen::FileExplorer),
                AppInputEvent::FileExplorerSelect,
            ),
            context_event(
                KeyCode::Backspace,
                KeyModifiers::NONE,
                InputContext::Screen(Screen::FileExplorer),
                AppInputEvent::FileExplorerBack,
            ),
            context_event(
                KeyCode::BackTab,
                KeyModifiers::SHIFT,
                InputContext::Mode(Mode::Browse),
                AppInputEvent::SwitchToEdit,
            ),
            context_event(
                KeyCode::BackTab,
                KeyModifiers::SHIFT,
                InputContext::Mode(Mode::Edit),
                AppInputEvent::SwitchToBrowse,
            ),
        ]
    }
}
