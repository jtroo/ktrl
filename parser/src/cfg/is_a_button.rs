pub(crate) fn is_a_button(osc: u16) -> bool {
    if cfg!(target_os = "windows") {
        matches!(osc, 1..=6 | 256..)
    } else {
        osc >= 256
    }
}

#[test]
fn mouse_inputs_most_care_about_are_considered_buttons() {
    use crate::keys::{OsCode, OsCode::*};
    const MOUSE_INPUTS: &[OsCode] = &[
        MouseWheelUp,
        MouseWheelDown,
        MouseWheelLeft,
        MouseWheelRight,
        BTN_LEFT,
        BTN_RIGHT,
        BTN_MIDDLE,
        BTN_SIDE,
        BTN_EXTRA,
        BTN_FORWARD,
        BTN_BACK,
    ];
    for input in MOUSE_INPUTS.iter().copied() {
        log::debug!("{input}");
        assert!(is_a_button(input.into()));
    }
}

#[test]
fn standard_keys_are_not_considered_buttons() {
    use crate::keys::{OsCode, OsCode::*};
    const KEY_INPUTS: &[OsCode] = &[
        KEY_0,
        KEY_1,
        KEY_2,
        KEY_3,
        KEY_4,
        KEY_5,
        KEY_6,
        KEY_7,
        KEY_8,
        KEY_9,
        KEY_A,
        KEY_B,
        KEY_C,
        KEY_D,
        KEY_E,
        KEY_F,
        KEY_G,
        KEY_H,
        KEY_I,
        KEY_J,
        KEY_K,
        KEY_L,
        KEY_M,
        KEY_N,
        KEY_O,
        KEY_P,
        KEY_Q,
        KEY_R,
        KEY_S,
        KEY_T,
        KEY_U,
        KEY_V,
        KEY_W,
        KEY_X,
        KEY_Y,
        KEY_Z,
        KEY_SEMICOLON,
        KEY_SLASH,
        KEY_GRAVE,
        KEY_LEFTBRACE,
        KEY_BACKSLASH,
        KEY_RIGHTBRACE,
        KEY_APOSTROPHE,
        KEY_MINUS,
        KEY_DOT,
        KEY_EQUAL,
        KEY_BACKSPACE,
        KEY_ESC,
        KEY_TAB,
        KEY_ENTER,
        KEY_LEFTCTRL,
        KEY_LEFTSHIFT,
        KEY_COMMA,
        KEY_RIGHTSHIFT,
        KEY_KPASTERISK,
        KEY_LEFTALT,
        KEY_SPACE,
        KEY_CAPSLOCK,
        KEY_F1,
        KEY_F2,
        KEY_F3,
        KEY_F4,
        KEY_F5,
        KEY_F6,
        KEY_F7,
        KEY_F8,
        KEY_F9,
        KEY_F10,
        KEY_F11,
        KEY_F12,
        KEY_NUMLOCK,
        KEY_CLEAR,
        KEY_SCROLLLOCK,
        KEY_KP0,
        KEY_KP1,
        KEY_KP2,
        KEY_KP3,
        KEY_KP4,
        KEY_KP5,
        KEY_KP6,
        KEY_KP7,
        KEY_KP8,
        KEY_KP9,
        KEY_KPMINUS,
        KEY_KPPLUS,
        KEY_KPDOT,
        KEY_KPENTER,
        KEY_RIGHTCTRL,
        KEY_KPSLASH,
        KEY_RIGHTALT,
        KEY_HOME,
        KEY_UP,
        KEY_PAGEUP,
        KEY_LEFT,
        KEY_RIGHT,
        KEY_END,
        KEY_DOWN,
        KEY_PAGEDOWN,
        KEY_INSERT,
        KEY_DELETE,
        KEY_MUTE,
        KEY_VOLUMEDOWN,
        KEY_VOLUMEUP,
        KEY_PAUSE,
        KEY_LEFTMETA,
        KEY_RIGHTMETA,
        KEY_COMPOSE,
        KEY_BACK,
        KEY_FORWARD,
        KEY_NEXTSONG,
        KEY_PLAYPAUSE,
        KEY_PREVIOUSSONG,
        KEY_STOP,
        KEY_HOMEPAGE,
        KEY_MAIL,
        KEY_MEDIA,
        KEY_REFRESH,
        KEY_F13,
        KEY_F14,
        KEY_F15,
        KEY_F16,
        KEY_F17,
        KEY_F18,
        KEY_F19,
        KEY_F20,
        KEY_F21,
        KEY_F22,
        KEY_F23,
        KEY_F24,
        KEY_HANGEUL,
        KEY_HANJA,
        KEY_252,
        KEY_102ND,
        KEY_PLAY,
        KEY_PRINT,
        KEY_SEARCH,
        KEY_FAVORITES,
        KEY_RO,
        KEY_HENKAN,
        KEY_MUHENKAN,
    ];
    for input in KEY_INPUTS.iter().copied() {
        log::debug!("{input}");
        assert!(!is_a_button(input.into()));
    }
}
