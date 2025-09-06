use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCodeSerializable {
    Char(char),
    Enter,
    Esc,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    BackTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyModifierSerializable {
    None,
    Control,
    Alt,
    Shift,
}

impl From<KeyCode> for KeyCodeSerializable {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => KeyCodeSerializable::Char(c),
            KeyCode::Enter => KeyCodeSerializable::Enter,
            KeyCode::Esc => KeyCodeSerializable::Esc,
            KeyCode::Tab => KeyCodeSerializable::Tab,
            KeyCode::Up => KeyCodeSerializable::Up,
            KeyCode::Down => KeyCodeSerializable::Down,
            KeyCode::Left => KeyCodeSerializable::Left,
            KeyCode::Right => KeyCodeSerializable::Right,
            KeyCode::Backspace => KeyCodeSerializable::Backspace,
            KeyCode::BackTab => KeyCodeSerializable::BackTab,
            _ => unimplemented!("Unsupported keycode: {:?}", code),
        }
    }
}

impl From<KeyCodeSerializable> for KeyCode {
    fn from(code: KeyCodeSerializable) -> Self {
        match code {
            KeyCodeSerializable::Char(c) => KeyCode::Char(c),
            KeyCodeSerializable::Enter => KeyCode::Enter,
            KeyCodeSerializable::Esc => KeyCode::Esc,
            KeyCodeSerializable::Tab => KeyCode::Tab,
            KeyCodeSerializable::Up => KeyCode::Up,
            KeyCodeSerializable::Down => KeyCode::Down,
            KeyCodeSerializable::Left => KeyCode::Left,
            KeyCodeSerializable::Right => KeyCode::Right,
            KeyCodeSerializable::Backspace => KeyCode::Backspace,
            KeyCodeSerializable::BackTab => KeyCode::BackTab,
        }
    }
}

impl From<KeyModifiers> for KeyModifierSerializable {
    fn from(modifier: KeyModifiers) -> Self {
        match modifier {
            KeyModifiers::NONE => KeyModifierSerializable::None,
            KeyModifiers::CONTROL => KeyModifierSerializable::Control,
            KeyModifiers::ALT => KeyModifierSerializable::Alt,
            KeyModifiers::SHIFT => KeyModifierSerializable::Shift,
            _ => unimplemented!("Unsupported key modifier: {:?}", modifier),
        }
    }
}

impl From<KeyModifierSerializable> for KeyModifiers {
    fn from(modifier: KeyModifierSerializable) -> Self {
        match modifier {
            KeyModifierSerializable::None => KeyModifiers::NONE,
            KeyModifierSerializable::Control => KeyModifiers::CONTROL,
            KeyModifierSerializable::Alt => KeyModifiers::ALT,
            KeyModifierSerializable::Shift => KeyModifiers::SHIFT,
        }
    }
}

impl KeyModifierSerializable {
    pub fn to_string(&self) -> String {
        match self {
            KeyModifierSerializable::None => "".into(),
            KeyModifierSerializable::Control => "CONTROL".into(),
            KeyModifierSerializable::Alt => "ALT".into(),
            KeyModifierSerializable::Shift => "SHIFT".into(),
        }
    }
}

impl KeyCodeSerializable {
    pub fn to_string(&self) -> String {
        match self {
            KeyCodeSerializable::Char(c) => c.to_string(),
            KeyCodeSerializable::Enter => "ENTER".into(),
            KeyCodeSerializable::Esc => "ESC".into(),
            KeyCodeSerializable::Tab => "TAB".into(),
            KeyCodeSerializable::Up => "UP".into(),
            KeyCodeSerializable::Down => "DOWN".into(),
            KeyCodeSerializable::Left => "LEFT".into(),
            KeyCodeSerializable::Right => "RIGHT".into(),
            KeyCodeSerializable::Backspace => "BACKSPACE".into(),
            KeyCodeSerializable::BackTab => "TAB".into(),
        }
    }
}
