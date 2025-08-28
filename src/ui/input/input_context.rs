use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    app::{PopUp, Screen},
    ui::input::key_bindings::{AppInputEvent, KeyBinding},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputContext {
    Screen(Screen),
    PopUp(PopUp),
    Global,
    /*
     * Might have app input mode here as well.
     * For instance, different input commands when searching for text vs inserting or browsing.
     */
}

pub fn get_input_contexts(screen: Screen, popup: PopUp) -> Vec<InputContext> {
    if popup != PopUp::None {
        vec![
            InputContext::PopUp(popup),
            InputContext::Screen(screen),
            InputContext::Global,
        ]
    } else {
        vec![InputContext::Screen(screen), InputContext::Global]
    }
}

pub fn context_event(
    key_code: KeyCode,
    modifier: KeyModifiers,
    context: InputContext,
    event: AppInputEvent
) -> ((InputContext, AppInputEvent), KeyBinding) {
    (
        (context, event),
        KeyBinding {
            key_code: key_code.into(),
            key_modifier: modifier.into(),
            context
        },
    )
}
