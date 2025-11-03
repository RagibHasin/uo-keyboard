// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

impl ITfFunctionProvider_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn GetType(&self) -> Result<GUID> {
        Ok(globals::IME_CLSID)
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn GetDescription(&self) -> Result<BSTR> {
        Err(E_NOTIMPL.into())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn GetFunction(&self, guid: *const GUID, iid: *const GUID) -> Result<IUnknown> {
        let guid = unsafe { guid.as_ref() }.unwrap();
        let iid = unsafe { iid.as_ref() }.unwrap();

        if guid == &GUID::zeroed() {
            let mut object = std::ptr::null_mut();
            unsafe { self.QueryInterface(iid, &mut object) }
                .ok()
                .map(|_| unsafe { IUnknown::from_raw(object) })
        } else {
            Err(E_NOINTERFACE.into())
        }
    }
}

impl ITfFunction_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn GetDisplayName(&self) -> Result<BSTR> {
        Err(E_NOTIMPL.into())
    }
}

impl ITfFnGetPreferredTouchKeyboardLayout_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn GetLayout(
        &self,
        layout_type: *mut TKBLayoutType,
        preferred_layout_id: *const u16,
    ) -> Result<()> {
        if !layout_type.is_null() && !preferred_layout_id.is_null() {
            unsafe {
                layout_type.write(TKBLT_OPTIMIZED);
                (preferred_layout_id as *mut u16).write(TKBL_UNDEFINED as _);
            }
            Ok(())
        } else {
            E_INVALIDARG.ok()
        }
    }
}
