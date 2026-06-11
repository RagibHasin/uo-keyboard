// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

impl ITfActiveLanguageProfileNotifySink_Impl for Ime_Impl {
    #[tracing::instrument(skip_all, ret, err)]
    fn OnActivated(&self, clsid: *const GUID, profile: *const GUID, activated: BOOL) -> Result<()> {
        let clsid = unsafe { clsid.as_ref() }.ok_or(S_OK)?;
        if activated.as_bool()
            && *clsid == globals::IME_CLSID
            && let Some(mut state) = self.state_mut()
        {
            let profile = unsafe { profile.as_ref() }.ok_or(S_OK)?;
            state.transcriber = transcriber::Transcriber::new(*profile);
        }
        Ok(())
    }
}
