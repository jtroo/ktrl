//! Platform specific code for low level keyboard read/write.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(all(
    feature = "simulated_output",
    not(feature = "simulated_input"),
    not(feature = "passthru_ahk"),
))]
mod simulated;
#[cfg(all(
    feature = "simulated_output",
    not(feature = "simulated_input"),
    not(feature = "passthru_ahk"),
))]
pub use simulated::*;
#[cfg(any(
    all(feature = "simulated_input", feature = "simulated_output"),
    all(
        feature = "simulated_input",
        feature = "simulated_output",
        feature = "passthru_ahk"
    ),
))]
mod sim_passthru;
#[cfg(any(
    all(feature = "simulated_input", feature = "simulated_output"),
    all(
        feature = "simulated_input",
        feature = "simulated_output",
        feature = "passthru_ahk"
    ),
))]
pub use sim_passthru::*;

pub const HI_RES_SCROLL_UNITS_IN_LO_RES: u16 = 120;

// ------------------ KeyValue --------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyValue {
    Release = 0,
    Press = 1,
    Repeat = 2,
    Tap,
    WakeUp,
}

impl From<i32> for KeyValue {
    fn from(item: i32) -> Self {
        match item {
            0 => Self::Release,
            1 => Self::Press,
            2 => Self::Repeat,
            _ => unreachable!(),
        }
    }
}

impl From<bool> for KeyValue {
    fn from(up: bool) -> Self {
        match up {
            true => Self::Release,
            false => Self::Press,
        }
    }
}

impl From<KeyValue> for bool {
    fn from(val: KeyValue) -> Self {
        matches!(val, KeyValue::Release)
    }
}

use kanata_parser::keys::OsCode;

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: OsCode,
    pub value: KeyValue,
}

#[allow(dead_code, unused)]
impl KeyEvent {
    pub fn new(code: OsCode, value: KeyValue) -> Self {
        Self { code, value }
    }
}

use core::fmt;
impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use kanata_keyberon::key_code::KeyCode;
        let direction = match self.value {
            KeyValue::Press => "↓",
            KeyValue::Release => "↑",
            KeyValue::Repeat => "⟳",
            KeyValue::Tap => "↕",
            KeyValue::WakeUp => "!",
        };
        let key_name = KeyCode::from(self.code);
        write!(f, "{}{:?}", direction, key_name)
    }
}
