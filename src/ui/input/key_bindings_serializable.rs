use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui::input::{
    input_context::InputContext,
    key_bindings::{AppInputEvent, KeyBinding},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextEventKey(pub InputContext, pub AppInputEvent);

#[derive(Deserialize, Serialize)]
pub struct KeyBindingsSerializable {
    pub by_event_type: HashMap<(InputContext, AppInputEvent), KeyBinding>,
}