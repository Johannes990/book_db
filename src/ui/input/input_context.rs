use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    app::{Mode, PopUp, Screen},
    ui::input::key_bindings::{AppInputEvent, KeyBinding},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputContext {
    Screen(Screen),
    PopUp(PopUp),
    Mode(Mode),
    Global,
}

pub fn get_input_contexts(screen: Screen, popup: PopUp, mode: Mode) -> Vec<InputContext> {
    if popup != PopUp::None {
        vec![
            InputContext::Mode(mode),
            InputContext::PopUp(popup),
            InputContext::Screen(screen),
            InputContext::Global,
        ]
    } else {
        vec![
            InputContext::Mode(mode),
            InputContext::Screen(screen),
            InputContext::Global,
        ]
    }
}

// this might need to be refactored to use a set of InputContexts later on
// so that we can pass in HashSet(InputContext::Global,
//                                InputContext::Editor::Editing,
//                                InputContext::Func(f))
// for example.
pub fn context_event(
    key_code: KeyCode,
    modifier: KeyModifiers,
    context: InputContext,
    event: AppInputEvent,
) -> ((InputContext, AppInputEvent), KeyBinding) {
    (
        (context, event),
        KeyBinding {
            key_code: key_code.into(),
            key_modifier: modifier.into(),
            context,
        },
    )
}
