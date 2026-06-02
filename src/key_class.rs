// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::core::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyClass {
    Letter,
    NumRow,
    NumPad,
    Symbol,
    Terminator,
    Backspace,
    Decimal,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyModifier {
    None,
    Shift,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyAction {
    Pass,
    OneShot,
    Append,
    End,
    Backspace,
    Cancel,
    AppendDot,
}

const SYMBOL_KEYS: &[VIRTUAL_KEY] = &[
    VK_OEM_1,
    VK_OEM_2,
    VK_OEM_3,
    VK_OEM_4,
    VK_OEM_5,
    VK_OEM_6,
    VK_OEM_7,
    VK_OEM_MINUS,
    VK_OEM_PLUS,
    VK_OEM_COMMA,
    VK_OEM_PERIOD,
];
const TERMINATOR_KEYS: &[VIRTUAL_KEY] = &[
    VK_TAB, VK_SPACE, VK_RETURN, VK_LEFT, VK_RIGHT, VK_UP, VK_DOWN, VK_HOME, VK_END, VK_PRIOR,
    VK_NEXT,
];

impl KeyClass {
    pub(crate) fn classify(key: u16) -> Self {
        if key == VK_BACK.0 {
            Self::Backspace
        } else if key == VK_DECIMAL.0 {
            Self::Decimal
        } else if is_key_in_range(key, VK_NUMPAD0, VK_NUMPAD9) {
            Self::NumPad
        } else if is_key_in_range(key, VK_0, VK_9) {
            Self::NumRow
        } else if is_key_in_range(key, VK_A, VK_Z) {
            Self::Letter
        } else if matches_key(key, SYMBOL_KEYS) {
            Self::Symbol
        } else if matches_key(key, TERMINATOR_KEYS) {
            Self::Terminator
        } else {
            Self::Function
        }
    }
}

impl KeyModifier {
    fn load() -> Self {
        if is_active(VK_CONTROL) | is_active(VK_MENU) | is_active(VK_LWIN) | is_active(VK_RWIN) {
            Self::Other
        } else if is_active(VK_SHIFT) {
            Self::Shift
        } else {
            Self::None
        }
    }
}

impl KeyAction {
    // #[tracing::instrument(ret)]
    pub(crate) fn classify(key: u16, composing: bool) -> Self {
        let caps_locked = unsafe { GetKeyState(VK_CAPITAL.0 as _) } & 1 == 1;
        let modifier = KeyModifier::load();

        let class = KeyClass::classify(key);

        tracing::trace!(caps_locked, ?modifier, ?class);

        use KeyClass::*;
        use KeyModifier::*;

        match (class, modifier, composing) {
            (Function, _, _) => Self::Pass,

            (Letter, Other, _) => Self::Pass,
            (Letter, None | Shift, _) => Self::Append,

            (Symbol, _, false) => Self::Pass,
            (Symbol, Other, true) => Self::Pass,
            (Symbol, None | Shift, true) => Self::Append,

            (NumRow, None, false) => Self::OneShot,
            (NumRow, Shift | Other, false) => Self::Pass,
            (NumRow, Other, true) => Self::Pass,
            (NumRow, None | Shift, true) => Self::Append,

            (NumPad, None, false) => Self::OneShot,
            (NumPad, None, true) => Self::Append,
            (NumPad, Shift | Other, _) => Self::Pass,

            (Decimal, None, true) => Self::AppendDot,
            (Decimal, _, _) => Self::Pass,

            (Backspace, _, false) => Self::Pass,
            (Backspace, None, true) => Self::Backspace,
            (Backspace, Shift | Other, true) => Self::Cancel,

            (Terminator, _, false) => Self::Pass,
            (Terminator, _, true) => Self::End,
        }
    }
}

fn is_key_in_range(key: u16, start: VIRTUAL_KEY, end: VIRTUAL_KEY) -> bool {
    (start.0..=end.0).contains(&key)
}

pub(crate) fn is_active(key: VIRTUAL_KEY) -> bool {
    (unsafe { GetAsyncKeyState(key.0 as _) } & i16::MIN == i16::MIN)
}

pub(crate) fn convert_vkey(code: u32) -> Result<u8> {
    let scan_code = unsafe { MapVirtualKeyW(code, MAPVK_VK_TO_VSC) };

    let mut keyboard_state = [0u8; 256];
    unsafe { GetKeyboardState(&mut keyboard_state) }?;

    let layout = unsafe { GetKeyboardLayout(0) };

    let mut ch = 0;
    let count = unsafe {
        ToUnicodeEx(
            code,
            scan_code,
            &keyboard_state,
            std::slice::from_mut(&mut ch),
            0,
            Some(layout),
        )
    };

    tracing::trace!(ch, wch = %(ch as u8 as char));

    (count == 1).then_some(ch as _).ok_or(S_FALSE.into())
}

fn matches_key(key: u16, keys: &[VIRTUAL_KEY]) -> bool {
    keys.iter().any(|vk| key == vk.0)
}
