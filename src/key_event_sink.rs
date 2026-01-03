// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

use key_class::KeyClass;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

impl Ime_Impl {
    // This theoretically makes sense, but a confirmation is needed whether a real
    // world situation exists where this becomes true.
    // #[tracing::instrument(skip_all, ret)]
    pub(crate) fn is_keyboard_disabled(&self) -> bool {
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
        if self.is_keyboard_disabled() {
            return Ok(FALSE);
        }

        let class = KeyClass::classify(wparam.0 as _);
        Ok((class != KeyClass::Modifier
            && (matches!(
                class,
                KeyClass::CompositeConvertible(_)
                    | KeyClass::FreeConvertible(_)
                    | KeyClass::NumPad(_)
            ) || self.composition().is_some()))
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

    #[tracing::instrument(skip_all, ret, err)]
    fn OnKeyDown(&self, ctx: Ref<'_, ITfContext>, wparam: WPARAM, lparam: LPARAM) -> Result<BOOL> {
        if self.is_keyboard_disabled() {
            return Ok(FALSE);
        }

        let class = KeyClass::classify(wparam.0 as _);
        if class == KeyClass::Delimiter {
            self.finish_composition(ctx.as_ref())?;

            let ki = KEYBDINPUT {
                wVk: VIRTUAL_KEY(wparam.0 as _),
                wScan: ((lparam.0 as usize >> 16) as _),
                dwFlags: KEYBD_EVENT_FLAGS::default(),
                time: 0,
                dwExtraInfo: 0,
            };
            let simulated_inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 { ki },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            dwFlags: KEYEVENTF_KEYUP,
                            ..ki
                        },
                    },
                },
            ];

            unsafe { SendInput(&simulated_inputs, std::mem::size_of::<INPUT>() as _) };

            return Ok(TRUE);
        }
        let class = if let KeyClass::NumPad(n) = class
            && n != b'.'
        {
            KeyClass::FreeConvertible(n)
        } else {
            class
        };

        let is_composing = self.composition().is_some();
        let is_eaten = match class {
            KeyClass::Modifier => FALSE,
            KeyClass::Unprocessed(_) | KeyClass::Backspace if !is_composing => FALSE,
            KeyClass::FreeConvertible(ch) if !is_composing => {
                self.add_single_char(ctx.unwrap(), ch)?;
                TRUE
            }
            KeyClass::CompositeConvertible(ch)
            | KeyClass::FreeConvertible(ch)
            | KeyClass::Unprocessed(ch) => {
                self.append_char_to_composition(ctx.unwrap(), ch)?;
                TRUE
            }
            KeyClass::Backspace => {
                self.pop_char_from_composition(ctx.unwrap())?;
                TRUE
            }
            KeyClass::NumPad(b'.') => {
                self.append_char_to_composition(ctx.unwrap(), b'.')?;
                self.append_char_to_composition(ctx.unwrap(), b'`')?;
                TRUE
            }
            KeyClass::NumPad(_) | KeyClass::Delimiter => unreachable!(),
        };

        Ok(is_eaten)
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
