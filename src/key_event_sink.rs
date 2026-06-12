// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

use key_class::{KeyAction, is_active};
use windows::Win32::UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::GetMessageExtraInfo};

const SYNTH: usize = 0x746e7953;

impl Ime_Impl {
    // This theoretically makes sense, but a confirmation is needed whether a real
    // world situation exists where this becomes true.
    // #[tracing::instrument(skip_all, ret)]
    fn is_keyboard_disabled(&self) -> bool {
        let state = self.state().unwrap();

        unsafe { state.thread_mgr.GetFocus() }.is_ok_and(|m| unsafe { m.GetTop() }.is_err())
            && compartment::read_bool(&state.thread_mgr, GUID_COMPARTMENT_KEYBOARD_DISABLED)
            && compartment::read_bool(&state.thread_mgr, GUID_COMPARTMENT_EMPTYCONTEXT)
    }
}

impl ITfKeyEventSink_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn OnSetFocus(&self, _: BOOL) -> Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip_all, ret, err)]
    fn OnTestKeyDown(&self, _: Ref<'_, ITfContext>, wparam: WPARAM, _: LPARAM) -> Result<BOOL> {
        Ok((!self.is_keyboard_disabled()
            && unsafe { GetMessageExtraInfo() } != LPARAM(SYNTH.cast_signed())
            && !matches!(
                (
                    KeyAction::classify(wparam.0 as _, self.composition().is_some()),
                    convert_vkey(wparam.0 as _),
                ),
                (KeyAction::Pass, _) | (KeyAction::OneShot | KeyAction::Append, Err(_))
            ))
        .into())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnTestKeyUp(
        &self,
        ctx: Ref<'_, ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        self.OnKeyUp(ctx, wparam, lparam)
    }

    #[tracing::instrument(skip(self, ctx), ret, err)]
    fn OnKeyDown(&self, ctx: Ref<'_, ITfContext>, wparam: WPARAM, lparam: LPARAM) -> Result<BOOL> {
        let composing = self.composition().is_some();
        let is_eaten = !self.is_keyboard_disabled()
            && unsafe { GetMessageExtraInfo() } != LPARAM(SYNTH.cast_signed())
            && match (
                KeyAction::classify(wparam.0 as _, composing),
                convert_vkey(wparam.0 as _),
            ) {
                (KeyAction::Pass, _) | (KeyAction::OneShot | KeyAction::Append, Err(_)) => false,
                (KeyAction::OneShot, Ok(ch)) => {
                    self.add_single_char(ctx.unwrap(), ch)?;
                    true
                }
                (KeyAction::Append, Ok(ch)) => {
                    self.append_char_to_composition(ctx.unwrap(), ch)?;
                    true
                }
                (KeyAction::End, _) => {
                    self.finish_composition(ctx.as_ref())?;
                    synthesize_key_input(wparam, lparam);
                    true
                }
                (KeyAction::Backspace, _) => {
                    self.pop_char_from_composition(ctx.unwrap())?;
                    true
                }
                (KeyAction::Cancel, _) => {
                    self.cancel_composition(ctx.unwrap())?;
                    true
                }
                (KeyAction::AppendDot, _) => {
                    self.append_char_to_composition(ctx.unwrap(), b'.')?;

                    let trailer = self.state().unwrap().transcriber.dot_trailer();
                    self.append_char_to_composition(ctx.unwrap(), trailer)?;
                    true
                }
            };

        Ok(is_eaten.into())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnKeyUp(&self, _: Ref<'_, ITfContext>, _: WPARAM, _: LPARAM) -> Result<BOOL> {
        Ok(FALSE)
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnPreservedKey(&self, _: Ref<'_, ITfContext>, _: *const GUID) -> Result<BOOL> {
        Ok(FALSE)
    }
}

fn convert_vkey(code: u32) -> Result<u8> {
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

#[tracing::instrument]
fn synthesize_key_input(wparam: WPARAM, lparam: LPARAM) {
    let shifted = is_active(VK_SHIFT);
    let ctrled = is_active(VK_CONTROL);
    let alted = is_active(VK_MENU);
    let metaed = is_active(VK_LWIN) || is_active(VK_RWIN);

    tracing::trace!(shifted, ctrled, alted, metaed);

    let modifiers = [
        (metaed, VK_LWIN),
        (ctrled, VK_CONTROL),
        (alted, VK_MENU),
        (shifted, VK_SHIFT),
    ];

    let active_keys = [
        make_input(
            VIRTUAL_KEY(wparam.0 as _),
            (lparam.0.cast_unsigned() >> 16) as _,
            KEYBD_EVENT_FLAGS::default(),
        ),
        make_input(
            VIRTUAL_KEY(wparam.0 as _),
            (lparam.0.cast_unsigned() >> 16) as _,
            KEYEVENTF_KEYUP,
        ),
    ];
    let _keys =
        modifiers
            .into_iter()
            .filter_map(|(is_active, key)| {
                is_active.then(|| make_input(key, 0, KEYBD_EVENT_FLAGS::default()))
            })
            .chain(active_keys)
            .chain(modifiers.into_iter().rev().filter_map(|(is_active, key)| {
                is_active.then(|| make_input(key, 0, KEYEVENTF_KEYUP))
            }))
            .collect::<Vec<_>>();

    unsafe { SendInput(&active_keys, std::mem::size_of::<INPUT>() as _) };
}

fn make_input(key: VIRTUAL_KEY, scan: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: SYNTH,
            },
        },
    }
}
