// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use std::sync::atomic::{AtomicPtr, Ordering::Relaxed};

use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
use windows::Win32::System::{SystemServices::*, Threading::*};

use crate::*;

static DLL_INSTANCE: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static mut CS: CRITICAL_SECTION = unsafe { core::mem::zeroed() };

pub(crate) fn instance_handle() -> HMODULE {
    HMODULE(DLL_INSTANCE.load(Relaxed))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "system" fn DllRegisterServer() -> HRESULT {
    fn register() -> Result<()> {
        let dll_instance = instance_handle();
        registration::register_server(dll_instance)
            .map_err(|_| Error::new(E_FAIL, "Failed to register server"))?;
        registration::register_profile(dll_instance)?;
        registration::register_categories()?;
        tracing::trace!("registration complete");
        Ok(())
    }

    // See https://github.com/rust-lang/rust-clippy/issues/13185
    #[allow(clippy::manual_inspect)]
    register()
        .map_err(|err| {
            let _ = unsafe { DllUnregisterServer() };
            err
        })
        .into()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "system" fn DllUnregisterServer() -> HRESULT {
    registration::unregister_profile().ok();
    registration::unregister_categories().ok();
    registration::unregister_server().ok();
    tracing::trace!("unregistration complete");
    S_OK
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "system" fn DllMain(
    instance: HINSTANCE,
    reason: u32,
    _lpv_reserved: *mut core::ffi::c_void,
) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            DLL_INSTANCE.store(instance.0, Relaxed);
            if (unsafe { InitializeCriticalSectionAndSpinCount(&raw mut CS, 0) }).is_err() {
                return false.into();
            }
            std::panic::set_hook(Box::new(tracing_panic::panic_hook));
            let trace_filter = tracing_subscriber::EnvFilter::new("error,uo_keyboard=trace");
            tracing_subscriber::registry()
                .with(
                    tracing_etw::LayerBuilder::new("UoBanglaKeyboard")
                        .build()
                        .unwrap()
                        .with_filter(trace_filter),
                )
                .init();
            tracing::trace!("Uo Bangla Keyboard loaded");
        }
        DLL_PROCESS_DETACH => {
            unsafe { DeleteCriticalSection(&raw mut CS) };
            tracing::trace!("Uo Bangla Keyboard unloaded");
        }
        _ => {}
    }
    true.into()
}
