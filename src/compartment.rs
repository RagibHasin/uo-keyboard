// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

#[derive(Debug, Clone)]
struct Compartment {
    thread_mgr: ITfThreadMgr,
    guid: GUID,
}

impl Compartment {
    fn new(thread_mgr: &ITfThreadMgr, guid: GUID) -> Compartment {
        Compartment {
            thread_mgr: thread_mgr.clone(),
            guid,
        }
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn get_compartment(&self) -> Result<ITfCompartment> {
        let manager = self.thread_mgr.cast::<ITfCompartmentMgr>()?;
        unsafe { manager.GetCompartment(&self.guid) }
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn get_bool(&self) -> Result<bool> {
        // Variant itself directly supports booleans but Windows expects i32 for compartment.
        Ok(self.get_u32()? != 0)
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn get_u32(&self) -> Result<u32> {
        unsafe {
            let variant = self.get_compartment()?.GetValue()?;
            // Windows expects i32 for compartment.
            Ok(i32::try_from(&variant)? as u32)
        }
    }
}

// #[tracing::instrument(skip_all, ret)]
pub(crate) fn read_bool(thread_mgr: &ITfThreadMgr, guid: GUID) -> bool {
    Compartment::new(thread_mgr, guid)
        .get_bool()
        .unwrap_or(false)
}
