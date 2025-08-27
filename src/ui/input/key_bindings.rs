use std::{collections::HashMap, fs, io};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{app::{PopUp, Screen}, log::log, ui::input::{input_context::{get_input_contexts, InputContext}, key_events_serializable::{KeyCodeSerializable, KeyModifierSerializable}}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key_code: KeyCodeSerializable,
    pub key_modifier: KeyModifierSerializable,
    pub context: InputContext,
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
    ClosePopUp,                 // close popup meaning switch to PopUp::None
    OpenQuitAppPopUp,           // open quit app popup
    QuitAppConfirm,             // confirm quit while in quit app
    MoveUpPrimary,              // go up in primary table or in options
    MoveDownPrimary,            // go down in primary table or in options
    MoveUpSecondary,            // go up in secondary table or in colorschemes
    MoveDownSecondary,          // go down in secondary table or in coloschemes
    ExecuteAction,              // execute current popup or SQL action
    ToggleOption,               // toggle selected option on/off
    FileExplorerSelect,         // select folder or file to load
    FileExplorerBack,           // go up to paren folder in file explorer
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
    pub by_event_input: HashMap<(InputContext, KeyBinding), AppInputEvent>,
}

impl KeyBindings {
    pub fn default() -> Self {
        let defaults: Vec<((InputContext, AppInputEvent), KeyBinding)> = Self::get_default_bindings();
        let mut bindings = Vec::new();
        let mut by_event_input = HashMap::new();

        for ((context, event), binding) in defaults {
            bindings.push(KeyBindingEntry {
                context,
                event,
                binding,
            });
            by_event_input.insert((context, binding), event);
        }

        Self {
            config: KeyBindingsSerializable { bindings },
            by_event_input
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
            

            let by_event_input = config
                .bindings
                .iter()
                .map(|entry| ((entry.context, entry.binding), entry.event))
                .collect();

            Ok(Self { config, by_event_input })
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

    pub fn resolve_event(&self, current_screen: Screen, current_popup: PopUp, key_event: KeyEvent) -> Option<AppInputEvent> {
        let log_msg = format!(
            "resolving input event: {:?}, current popup: {:?}, current screen: {:?}",
            key_event, current_popup, current_screen
        );
        log(&log_msg);

        let binding = KeyBinding {
            key_code: key_event.code.into(),
            key_modifier: key_event.modifiers.into(),
            context: InputContext::Global,
        };

        let contexts = get_input_contexts(current_screen, current_popup);

        for context in contexts {
            let binding_with_context = KeyBinding {context, ..binding };

            if let Some(event) = self.by_event_input.get(&(context, binding_with_context)) {
                log(&format!("Found event {:?} in context {:?}", event, context));
                return Some(*event);
            }
        }

        log("No matching event found in any context");
        None
    }

    fn get_default_bindings() -> Vec<((InputContext, AppInputEvent), KeyBinding)> {
        vec![
            ((InputContext::Global, AppInputEvent::OpenSplashScreen), KeyBinding {
                key_code: KeyCode::Char('s').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global
            }),
            ((InputContext::Global, AppInputEvent::OpenFileExplorerScreen), KeyBinding {
                key_code: KeyCode::Char('f').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::OpenDBSchemaScreen), KeyBinding {
                key_code: KeyCode::Char('d').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::OpenDBTableScreen), KeyBinding {
                key_code: KeyCode::Char('t').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::OpenCreateNewFileScreen), KeyBinding {
                key_code: KeyCode::Char('c').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::OpenOptionsScreen), KeyBinding {
                key_code: KeyCode::Char('o').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Screen(Screen::DataBaseTable), AppInputEvent::OpenInsertRowPopUp), KeyBinding {
                key_code: KeyCode::Char('i').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::DataBaseTable),
            }),
            ((InputContext::Screen(Screen::DataBaseTable), AppInputEvent::OpenDeleteRowPopUp), KeyBinding {
                key_code: KeyCode::Char('d').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::DataBaseTable),
            }),
            ((InputContext::Screen(Screen::DatabaseSchema), AppInputEvent::OpenInsertTablePopUp), KeyBinding {
                key_code: KeyCode::Char('i').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::DatabaseSchema),
            }),
            ((InputContext::Screen(Screen::DatabaseSchema), AppInputEvent::OpenDeleteTablePopUp), KeyBinding {
                key_code: KeyCode::Char('d').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::DatabaseSchema),
            }),
            ((InputContext::Global, AppInputEvent::ClosePopUp), KeyBinding {
                key_code: KeyCode::Esc.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::OpenQuitAppPopUp), KeyBinding {
                key_code: KeyCode::Char('q').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::PopUp(PopUp::Quit), AppInputEvent::QuitAppConfirm), KeyBinding {
                key_code: KeyCode::Char('y').into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::PopUp(PopUp::Quit),
            }),
            ((InputContext::Global, AppInputEvent::MoveUpPrimary), KeyBinding {
                key_code: KeyCode::Up.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::MoveDownPrimary), KeyBinding {
                key_code: KeyCode::Down.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::MoveUpSecondary), KeyBinding {
                key_code: KeyCode::Left.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::MoveDownSecondary), KeyBinding {
                key_code: KeyCode::Right.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Global, AppInputEvent::ExecuteAction), KeyBinding {
                key_code: KeyCode::Enter.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Global,
            }),
            ((InputContext::Screen(Screen::Options), AppInputEvent::ToggleOption), KeyBinding {
                key_code: KeyCode::Tab.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::Options),
            }),
            ((InputContext::Screen(Screen::FileExplorer), AppInputEvent::FileExplorerSelect), KeyBinding {
                key_code: KeyCode::Enter.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::FileExplorer),
            }),
            ((InputContext::Screen(Screen::FileExplorer), AppInputEvent::FileExplorerBack), KeyBinding {
                key_code: KeyCode::Backspace.into(),
                key_modifier: KeyModifiers::NONE.into(),
                context: InputContext::Screen(Screen::FileExplorer),
            }),
        ]
    }
}
