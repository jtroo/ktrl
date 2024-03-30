use kanata_parser::keys::OsCode;

#[allow(unused)]
pub(crate) fn u16_to_osc(input: u16) -> Option<OsCode> {
    Some(if input < 0xE000 {
        match input {
            0x01 => OsCode::KEY_ESC,
            0x02 => OsCode::KEY_1,
            0x03 => OsCode::KEY_2,
            0x04 => OsCode::KEY_3,
            0x05 => OsCode::KEY_4,
            0x06 => OsCode::KEY_5,
            0x07 => OsCode::KEY_6,
            0x08 => OsCode::KEY_7,
            0x09 => OsCode::KEY_8,
            0x0A => OsCode::KEY_9,
            0x0B => OsCode::KEY_0,
            0x0C => OsCode::KEY_MINUS,
            0x0D => OsCode::KEY_EQUAL,
            0x0E => OsCode::KEY_BACKSPACE,
            0x0F => OsCode::KEY_TAB,
            0x10 => OsCode::KEY_Q,
            0x11 => OsCode::KEY_W,
            0x12 => OsCode::KEY_E,
            0x13 => OsCode::KEY_R,
            0x14 => OsCode::KEY_T,
            0x15 => OsCode::KEY_Y,
            0x16 => OsCode::KEY_U,
            0x17 => OsCode::KEY_I,
            0x18 => OsCode::KEY_O,
            0x19 => OsCode::KEY_P,
            0x1A => OsCode::KEY_LEFTBRACE,
            0x1B => OsCode::KEY_RIGHTBRACE,
            0x1C => OsCode::KEY_ENTER,
            0x1D => OsCode::KEY_LEFTCTRL,
            0x1E => OsCode::KEY_A,
            0x1F => OsCode::KEY_S,
            0x20 => OsCode::KEY_D,
            0x21 => OsCode::KEY_F,
            0x22 => OsCode::KEY_G,
            0x23 => OsCode::KEY_H,
            0x24 => OsCode::KEY_J,
            0x25 => OsCode::KEY_K,
            0x26 => OsCode::KEY_L,
            0x27 => OsCode::KEY_SEMICOLON,
            0x28 => OsCode::KEY_APOSTROPHE,
            0x29 => OsCode::KEY_GRAVE,
            0x2A => OsCode::KEY_LEFTSHIFT,
            0x2B => OsCode::KEY_BACKSLASH,
            0x2C => OsCode::KEY_Z,
            0x2D => OsCode::KEY_X,
            0x2E => OsCode::KEY_C,
            0x2F => OsCode::KEY_V,
            0x30 => OsCode::KEY_B,
            0x31 => OsCode::KEY_N,
            0x32 => OsCode::KEY_M,
            0x33 => OsCode::KEY_COMMA,
            0x34 => OsCode::KEY_DOT,
            0x35 => OsCode::KEY_SLASH,
            0x36 => OsCode::KEY_RIGHTSHIFT,
            0x37 => OsCode::KEY_KPASTERISK,
            0x38 => OsCode::KEY_LEFTALT,
            0x39 => OsCode::KEY_SPACE,
            0x3A => OsCode::KEY_CAPSLOCK,
            0x3B => OsCode::KEY_F1,
            0x3C => OsCode::KEY_F2,
            0x3D => OsCode::KEY_F3,
            0x3E => OsCode::KEY_F4,
            0x3F => OsCode::KEY_F5,
            0x40 => OsCode::KEY_F6,
            0x41 => OsCode::KEY_F7,
            0x42 => OsCode::KEY_F8,
            0x43 => OsCode::KEY_F9,
            0x44 => OsCode::KEY_F10,
            0x45 => OsCode::KEY_NUMLOCK,
            0x46 => OsCode::KEY_SCROLLLOCK,
            0x47 => OsCode::KEY_KP7,
            0x48 => OsCode::KEY_KP8,
            0x49 => OsCode::KEY_KP9,
            0x4A => OsCode::KEY_KPMINUS,
            0x4B => OsCode::KEY_KP4,
            0x4C => OsCode::KEY_KP5,
            0x4D => OsCode::KEY_KP6,
            0x4E => OsCode::KEY_KPPLUS,
            0x4F => OsCode::KEY_KP1,
            0x50 => OsCode::KEY_KP2,
            0x51 => OsCode::KEY_KP3,
            0x52 => OsCode::KEY_KP0,
            0x53 => OsCode::KEY_KPDOT,
            0x56 => OsCode::KEY_102ND, /* Key between the left shift and Z. */
            0x57 => OsCode::KEY_F11,
            0x58 => OsCode::KEY_F12,
            0x64 => OsCode::KEY_F13,
            0x65 => OsCode::KEY_F14,
            0x66 => OsCode::KEY_F15,
            0x67 => OsCode::KEY_F16,
            0x68 => OsCode::KEY_F17,
            0x69 => OsCode::KEY_F18,
            0x6A => OsCode::KEY_F19,
            0x6B => OsCode::KEY_F20,
            0x6C => OsCode::KEY_F21,
            0x6D => OsCode::KEY_F22,
            0x6E => OsCode::KEY_F23,
            0x76 => OsCode::KEY_F24,
            0x70 => OsCode::KEY_KATAKANA,
            // Note: the OEM keys below don't seem to correspond to the same VK OEM
            // mappings as the LLHOOK codes.
            // ScanCode::Oem1 = 0x5A, /* VK_OEM_WSCTRL */
            // ScanCode::Oem2 = 0x5B, /* VK_OEM_FINISH */
            // ScanCode::Oem3 = 0x5C, /* VK_OEM_JUMP */
            // ScanCode::Oem4 = 0x5E, /* VK_OEM_BACKTAB */
            // ScanCode::Oem5 = 0x5F, /* VK_OEM_AUTO */
            // ScanCode::Oem6 = 0x6F, /* VK_OEM_PA3 */
            // ScanCode::Oem7 = 0x71, /* VK_OEM_RESET */
            // ScanCode::EraseEOF = 0x5D,
            // ScanCode::Zoom => 0x62,
            // ScanCode::Help => 0x63,
            // ScanCode::AltPrintScreen = 0x55, /* Alt + print screen. */
            // ScanCode::SBCSChar = 0x77,
            // ScanCode::Convert = 0x79,
            // ScanCode::NonConvert = 0x7B,
            _ => return None,
        }
    } else {
        match input & 0xFF {
            0x10 => OsCode::KEY_PREVIOUSSONG,
            0x19 => OsCode::KEY_NEXTSONG,
            0x1C => OsCode::KEY_KPENTER,
            0x1D => OsCode::KEY_RIGHTCTRL,
            0x20 => OsCode::KEY_MUTE,
            0x22 => OsCode::KEY_PLAYPAUSE, // sc_media_play
            // 0x24 => OsCode::KEY_TODO, // sc_media_stop
            0x2E => OsCode::KEY_VOLUMEDOWN, // sc_volume_down
            0x30 => OsCode::KEY_VOLUMEUP,   // sc_volume_up
            // 0x32 => OsCode::KEY_TODO, // sc_browser_home
            0x35 => OsCode::KEY_KPSLASH,  // sc_numpad_divide
            0x37 => OsCode::KEY_PRINT,    // sc_printScreen
            0x38 => OsCode::KEY_RIGHTALT, // sc_altRight
            // 0x46 => OsCode::KEY_TODO, // sc_cancel
            0x47 => OsCode::KEY_HOME,      // sc_home
            0x48 => OsCode::KEY_UP,        // sc_arrowUp
            0x49 => OsCode::KEY_PAGEUP,    // sc_pageUp
            0x4B => OsCode::KEY_LEFT,      // sc_arrowLeft
            0x4D => OsCode::KEY_RIGHT,     // sc_arrowRight
            0x4F => OsCode::KEY_END,       // sc_end
            0x50 => OsCode::KEY_DOWN,      // sc_arrowDown
            0x51 => OsCode::KEY_PAGEDOWN,  // sc_pageDown
            0x52 => OsCode::KEY_INSERT,    // sc_insert
            0x53 => OsCode::KEY_DELETE,    // sc_delete
            0x5B => OsCode::KEY_LEFTMETA,  // sc_metaLeft
            0x5C => OsCode::KEY_RIGHTMETA, // sc_metaRight
            0x5D => OsCode::KEY_COMPOSE,   // sc_application / compose
            // 0x5E => OsCode::KEY_TODO, // sc_power
            // 0x5F => OsCode::KEY_TODO, // sc_sleep
            // 0x63 => OsCode::KEY_TODO, // sc_wake
            // 0x65 => OsCode::KEY_TODO, // sc_browser_search
            // 0x66 => OsCode::KEY_TODO, // sc_browser_favorites
            // 0x67 => OsCode::KEY_TODO, // sc_browser_refresh
            // 0x68 => OsCode::KEY_TODO, // sc_browser_stop
            0x69 => OsCode::KEY_FORWARD, // sc_browser_forward
            0x6A => OsCode::KEY_BACK,    // sc_browser_back
            // 0x6B => OsCode::KEY_TODO, // sc_launch_app1
            // 0x6C => OsCode::KEY_TODO, // sc_launch_email
            // 0x6D => OsCode::KEY_TODO, // sc_launch_media
            _ => return None,
        }
    })
}

#[allow(unused)]
pub(crate) fn osc_to_u16(osc: OsCode) -> Option<u16> {
    Some(match osc {
        OsCode::KEY_ESC => 0x01,
        OsCode::KEY_1 => 0x02,
        OsCode::KEY_2 => 0x03,
        OsCode::KEY_3 => 0x04,
        OsCode::KEY_4 => 0x05,
        OsCode::KEY_5 => 0x06,
        OsCode::KEY_6 => 0x07,
        OsCode::KEY_7 => 0x08,
        OsCode::KEY_8 => 0x09,
        OsCode::KEY_9 => 0x0A,
        OsCode::KEY_0 => 0x0B,
        OsCode::KEY_MINUS => 0x0C,
        OsCode::KEY_EQUAL => 0x0D,
        OsCode::KEY_BACKSPACE => 0x0E,
        OsCode::KEY_TAB => 0x0F,
        OsCode::KEY_Q => 0x10,
        OsCode::KEY_W => 0x11,
        OsCode::KEY_E => 0x12,
        OsCode::KEY_R => 0x13,
        OsCode::KEY_T => 0x14,
        OsCode::KEY_Y => 0x15,
        OsCode::KEY_U => 0x16,
        OsCode::KEY_I => 0x17,
        OsCode::KEY_O => 0x18,
        OsCode::KEY_P => 0x19,
        OsCode::KEY_LEFTBRACE => 0x1A,
        OsCode::KEY_RIGHTBRACE => 0x1B,
        OsCode::KEY_ENTER => 0x1C,
        OsCode::KEY_LEFTCTRL => 0x1D,
        OsCode::KEY_A => 0x1E,
        OsCode::KEY_S => 0x1F,
        OsCode::KEY_D => 0x20,
        OsCode::KEY_F => 0x21,
        OsCode::KEY_G => 0x22,
        OsCode::KEY_H => 0x23,
        OsCode::KEY_J => 0x24,
        OsCode::KEY_K => 0x25,
        OsCode::KEY_L => 0x26,
        OsCode::KEY_SEMICOLON => 0x27,
        OsCode::KEY_APOSTROPHE => 0x28,
        OsCode::KEY_GRAVE => 0x29,
        OsCode::KEY_LEFTSHIFT => 0x2A,
        OsCode::KEY_BACKSLASH => 0x2B,
        OsCode::KEY_Z => 0x2C,
        OsCode::KEY_X => 0x2D,
        OsCode::KEY_C => 0x2E,
        OsCode::KEY_V => 0x2F,
        OsCode::KEY_B => 0x30,
        OsCode::KEY_N => 0x31,
        OsCode::KEY_M => 0x32,
        OsCode::KEY_COMMA => 0x33,
        OsCode::KEY_DOT => 0x34,
        OsCode::KEY_SLASH => 0x35,
        OsCode::KEY_RIGHTSHIFT => 0x36,
        OsCode::KEY_KPASTERISK => 0x37,
        OsCode::KEY_LEFTALT => 0x38,
        OsCode::KEY_SPACE => 0x39,
        OsCode::KEY_CAPSLOCK => 0x3A,
        OsCode::KEY_F1 => 0x3B,
        OsCode::KEY_F2 => 0x3C,
        OsCode::KEY_F3 => 0x3D,
        OsCode::KEY_F4 => 0x3E,
        OsCode::KEY_F5 => 0x3F,
        OsCode::KEY_F6 => 0x40,
        OsCode::KEY_F7 => 0x41,
        OsCode::KEY_F8 => 0x42,
        OsCode::KEY_F9 => 0x43,
        OsCode::KEY_F10 => 0x44,
        OsCode::KEY_NUMLOCK => 0x45,
        OsCode::KEY_SCROLLLOCK => 0x46,
        OsCode::KEY_KP7 => 0x47,
        OsCode::KEY_KP8 => 0x48,
        OsCode::KEY_KP9 => 0x49,
        OsCode::KEY_KPMINUS => 0x4A,
        OsCode::KEY_KP4 => 0x4B,
        OsCode::KEY_KP5 => 0x4C,
        OsCode::KEY_KP6 => 0x4D,
        OsCode::KEY_KPPLUS => 0x4E,
        OsCode::KEY_KP1 => 0x4F,
        OsCode::KEY_KP2 => 0x50,
        OsCode::KEY_KP3 => 0x51,
        OsCode::KEY_KP0 => 0x52,
        OsCode::KEY_KPDOT => 0x53,
        OsCode::KEY_102ND => 0x56, /* Key between the left shift and Z. */
        OsCode::KEY_F11 => 0x57,
        OsCode::KEY_F12 => 0x58,
        OsCode::KEY_F13 => 0x64,
        OsCode::KEY_F14 => 0x65,
        OsCode::KEY_F15 => 0x66,
        OsCode::KEY_F16 => 0x67,
        OsCode::KEY_F17 => 0x68,
        OsCode::KEY_F18 => 0x69,
        OsCode::KEY_F19 => 0x6A,
        OsCode::KEY_F20 => 0x6B,
        OsCode::KEY_F21 => 0x6C,
        OsCode::KEY_F22 => 0x6D,
        OsCode::KEY_F23 => 0x6E,
        OsCode::KEY_F24 => 0x76,
        OsCode::KEY_KATAKANA => 0x70,
        OsCode::KEY_PREVIOUSSONG => 0xE010,
        OsCode::KEY_NEXTSONG => 0xE019,
        OsCode::KEY_KPENTER => 0xE01C,
        OsCode::KEY_RIGHTCTRL => 0xE01D,
        OsCode::KEY_MUTE => 0xE020,
        OsCode::KEY_PLAYPAUSE => 0xE022,  // sc_media_play
        OsCode::KEY_VOLUMEDOWN => 0xE02E, // sc_volume_down
        OsCode::KEY_VOLUMEUP => 0xE030,   // sc_volume_up
        OsCode::KEY_KPSLASH => 0xE035,    // sc_numpad_divide
        OsCode::KEY_PRINT => 0xE037,      // sc_printScreen
        OsCode::KEY_RIGHTALT => 0xE038,   // sc_altRight
        OsCode::KEY_HOME => 0xE047,       // sc_home
        OsCode::KEY_UP => 0xE048,         // sc_arrowUp
        OsCode::KEY_PAGEUP => 0xE049,     // sc_pageUp
        OsCode::KEY_LEFT => 0xE04B,       // sc_arrowLeft
        OsCode::KEY_RIGHT => 0xE04D,      // sc_arrowRight
        OsCode::KEY_END => 0xE04F,        // sc_end
        OsCode::KEY_DOWN => 0xE050,       // sc_arrowDown
        OsCode::KEY_PAGEDOWN => 0xE051,   // sc_pageDown
        OsCode::KEY_INSERT => 0xE052,     // sc_insert
        OsCode::KEY_DELETE => 0xE053,     // sc_delete
        OsCode::KEY_LEFTMETA => 0xE05B,   // sc_metaLeft
        OsCode::KEY_RIGHTMETA => 0xE05C,  // sc_metaRight
        OsCode::KEY_COMPOSE => 0xE05D,    // sc_application / compose
        OsCode::KEY_FORWARD => 0xE069,    // sc_browser_forward
        OsCode::KEY_BACK => 0xE06A,       // sc_browser_back
        _ => return None,
    })
}
