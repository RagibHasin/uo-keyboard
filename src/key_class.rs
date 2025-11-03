// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyClass {
    CompositeConvertible(u8),
    FreeConvertible(u8),
    NumPad(u8),
    Unprocessed(u8),
    Backspace,
    Delimiter,
    Modifier,
}

impl KeyClass {
    #[tracing::instrument(ret)]
    pub(crate) fn classify(key: u16) -> KeyClass {
        let caps_locked = unsafe { GetKeyState(VK_CAPITAL.0 as _) } & 1 == 1;
        let shifted = unsafe { GetAsyncKeyState(VK_SHIFT.0 as _) } & i16::MIN == i16::MIN;
        let ctrled = unsafe { GetAsyncKeyState(VK_CONTROL.0 as _) } & i16::MIN == i16::MIN;
        let alted = unsafe { GetAsyncKeyState(VK_MENU.0 as _) } & i16::MIN == i16::MIN;
        let metaed = unsafe { GetAsyncKeyState(VK_LWIN.0 as _) } & i16::MIN == i16::MIN
            || unsafe { GetAsyncKeyState(VK_RWIN.0 as _) } & i16::MIN == i16::MIN;

        let ch = convert_vkey(key as _);

        tracing::trace!(caps_locked, shifted, ctrled, alted, metaed, ch);

        if ctrled
            || alted
            || metaed
            || [
                VK_SHIFT,
                VK_LSHIFT,
                VK_RSHIFT,
                VK_CONTROL,
                VK_LCONTROL,
                VK_RCONTROL,
                VK_MENU,
                VK_LMENU,
                VK_RMENU,
                VK_LWIN,
                VK_RWIN,
            ]
            .into_iter()
            .any(|vk| key == vk.0)
        {
            KeyClass::Modifier
        } else if (VK_A.0..=VK_Z.0).contains(&key) {
            let ch = key as u8;
            KeyClass::CompositeConvertible(if caps_locked || shifted {
                ch
            } else {
                ch.to_ascii_lowercase()
            })
        } else if (VK_0.0..=VK_9.0).contains(&key) {
            KeyClass::FreeConvertible(match (shifted, key as u8) {
                (true, b'1') => b'!',
                (true, b'2') => b'@',
                (true, b'3') => b'#',
                (true, b'4') => b'$',
                (true, b'5') => b'%',
                (true, b'6') => b'^',
                (true, b'7') => b'&',
                (true, b'8') => b'*',
                (true, b'9') => b'(',
                (true, b'0') => b')',
                (_, ch) => ch,
            })
        } else if key == VK_OEM_PERIOD.0 {
            KeyClass::FreeConvertible(b'.')
        } else if (VK_NUMPAD0.0..=VK_NUMPAD9.0).contains(&key) {
            KeyClass::NumPad((key - VK_NUMPAD0.0) as u8 + b'0')
        } else if key == VK_DECIMAL.0 {
            KeyClass::NumPad(b'.')
        } else if key == VK_TAB.0 || key == VK_SPACE.0 || key == VK_RETURN.0 {
            KeyClass::Delimiter
        } else if key == VK_BACK.0 {
            KeyClass::Backspace
        } else {
            match ch {
                0 => KeyClass::Delimiter,
                _ => KeyClass::Unprocessed(ch as _),
            }
        }
    }
}

fn convert_vkey(code: u32) -> u16 {
    let scan_code = unsafe { MapVirtualKeyW(code, MAPVK_VK_TO_VSC) };

    let mut keyboard_state = [0u8; 256];
    if unsafe { GetKeyboardState(&mut keyboard_state) }.is_err() {
        return 0;
    }

    let mut wch = 0;
    if unsafe {
        ToUnicode(
            code,
            scan_code,
            Some(&keyboard_state),
            std::slice::from_mut(&mut wch),
            0,
        )
    } == 1
    {
        return wch;
    }

    0
}
