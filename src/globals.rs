// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use windows::Win32::System::SystemServices::*;

use crate::*;

pub(crate) const IME_DESCRIPTION: &str = "Ũõ Keyboard";
pub(crate) const IME_LANGID: u16 = ((SUBLANG_BANGLA_BANGLADESH << 10) | LANG_BANGLA) as u16;
pub(crate) const IME_ICON_INDEX: u32 = -12i32 as u32;

pub(crate) const IME_CLSID: GUID = GUID::from_u128(0x9de5f508_1b88_42bc_9f58_be50828c40b1);
pub(crate) const IME_PROFILE: GUID = GUID::from_u128(0x5f9083f2_0f4a_4c6e_af95_12c7bfc1603e);
