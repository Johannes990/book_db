use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Serialize, Deserialize};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum KeyModifierSerializable {
    NONE = 0,
    CONTROL = 1,
    ALT = 2,
    SHIFT = 4,
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
        }
    }
}

impl From<KeyModifiers> for KeyModifierSerializable {
    fn from(modifier: KeyModifiers) -> Self {
        match modifier {
            KeyModifiers::NONE => KeyModifierSerializable::NONE,
            KeyModifiers::CONTROL => KeyModifierSerializable::CONTROL,
            KeyModifiers::ALT => KeyModifierSerializable::ALT,
            KeyModifiers::SHIFT => KeyModifierSerializable::SHIFT,
            _ => unimplemented!("Unsupported key modifier: {:?}", modifier),
        }
    }
}

impl From<KeyModifierSerializable> for KeyModifiers {
    fn from(modifier: KeyModifierSerializable) -> Self {
        match modifier {
            KeyModifierSerializable::NONE => KeyModifiers::NONE,
            KeyModifierSerializable::CONTROL => KeyModifiers::CONTROL,
            KeyModifierSerializable::ALT => KeyModifiers::ALT,
            KeyModifierSerializable::SHIFT => KeyModifiers::SHIFT,
        }
    }
} 
