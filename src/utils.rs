// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use std::{mem::ManuallyDrop, slice};

use crate::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TfSelection {
    pub(crate) range: Option<ITfRange>,
    pub(crate) style: TF_SELECTIONSTYLE,
}

impl From<TF_SELECTION> for TfSelection {
    fn from(selection: TF_SELECTION) -> TfSelection {
        TfSelection {
            range: ManuallyDrop::into_inner(selection.range),
            style: selection.style,
        }
    }
}

impl From<TfSelection> for TF_SELECTION {
    fn from(selection: TfSelection) -> TF_SELECTION {
        TF_SELECTION {
            range: ManuallyDrop::new(selection.range),
            style: selection.style,
        }
    }
}

pub(crate) fn get_selection(edit_cookie: u32, ctx: &ITfContext, index: u32) -> Result<TfSelection> {
    let mut selection = TF_SELECTION::default();
    let mut fetched = 0;
    unsafe {
        ctx.GetSelection(
            edit_cookie,
            index,
            slice::from_mut(&mut selection),
            &mut fetched,
        )
    }?;
    if fetched != 1 {
        S_FALSE.ok()?;
    }

    Ok(selection.into())
}

// #[tracing::instrument(skip_all, ret, err)]
pub(crate) fn set_selection(
    edit_cookie: u32,
    ctx: &ITfContext,
    selection: TfSelection,
) -> Result<()> {
    let selection = selection.into();
    unsafe { ctx.SetSelection(edit_cookie, slice::from_ref(&selection)) }?;
    ManuallyDrop::into_inner(selection.range);
    Ok(())
}

/// Returns `true` if `test` is entirely contained within `cover`.
#[tracing::instrument(skip_all, ret)]
pub(crate) fn is_range_covered(edit_cookie: u32, test: &ITfRange, cover: &ITfRange) -> bool {
    unsafe { cover.CompareStart(edit_cookie, test, TF_ANCHOR_START) }.is_ok_and(|r| r <= 0)
        && unsafe { cover.CompareEnd(edit_cookie, test, TF_ANCHOR_END) }.is_ok_and(|r| r >= 0)
}

pub(crate) fn create_instance_inproc<T: Interface>(clsid: &GUID) -> Result<T> {
    use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
    unsafe { CoCreateInstance(clsid, None, CLSCTX_INPROC_SERVER) }
}
