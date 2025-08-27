use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};

use crate::{app::{PopUp, Screen}, ui::input::key_bindings::KeyBinding};

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

pub fn get_input_contexts(screen: Screen, popup: PopUp, key_event: KeyEvent) -> Vec<InputContext> {
    let contexts = if popup != PopUp::None {
        vec![
            InputContext::PopUp(popup),
            InputContext::Screen(screen),
            InputContext::Global,
        ]
    } else {
        vec![
            InputContext::Screen(screen),
            InputContext::Global,
        ]
    };

    contexts
}
