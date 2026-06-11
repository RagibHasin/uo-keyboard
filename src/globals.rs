// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use windows::Win32::System::SystemServices::*;

use crate::*;

pub(crate) const IME_DESCRIPTION: &str = "Ũõ Keyboard";
pub(crate) const IME_LANGID: u16 = ((SUBLANG_BANGLA_BANGLADESH << 10) | LANG_BANGLA) as u16;
pub(crate) const IME_CLSID: GUID = GUID::from_u128(0x9de5f508_1b88_42bc_9f58_be50828c40b1);

pub(crate) const IME_PROFILE_AVRO: GUID = GUID::from_u128(0x3cbd54da_d734_46fe_8dfe_e963187e9f37);
pub(crate) const IME_PROFILE_DESCRIPTION_AVRO: &str = "Ũõ Keyboard (অভ্র)";
pub(crate) const IME_ICON_INDEX_AVRO: u32 = (-11i32).cast_unsigned();

pub(crate) const IME_PROFILE_KHIPRO: GUID = GUID::from_u128(0x5f9083f2_0f4a_4c6e_af95_12c7bfc1603e);
pub(crate) const IME_PROFILE_DESCRIPTION_KHIPRO: &str = "Ũõ Keyboard (ক্ষিপ্র)";
pub(crate) const IME_ICON_INDEX_KHIPRO: u32 = (-12i32).cast_unsigned();
