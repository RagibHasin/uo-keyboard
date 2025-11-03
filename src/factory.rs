// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Mutex, OnceLock};

use windows::Win32::System::Com::*;

use crate::*;

use globals::*;

pub(crate) static DLL_REF_COUNT: AtomicI32 = AtomicI32::new(-1);
pub(crate) static CLASS_FACTORY_OBJECT: Mutex<OnceLock<StaticComObject<ClassFactory>>> =
    Mutex::new(OnceLock::new());

pub(crate) fn dll_add_ref() {
    DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub(crate) fn dll_release() {
    if DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst) < 0 {
        free_global_objects();
    }
}

#[implement(IClassFactory)]
#[derive(Debug)]
pub(crate) struct ClassFactory;

impl IClassFactory_Impl for ClassFactory_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn CreateInstance(
        &self,
        outer: Ref<'_, IUnknown>,
        iid: *const GUID,
        object: *mut *mut c_void,
    ) -> Result<()> {
        if object.is_null() {
            return E_INVALIDARG.ok();
        }

        if outer.is_some() {
            return CLASS_E_NOAGGREGATION.ok();
        }

        let instance = Ime::new()?.into_object();
        unsafe { instance.QueryInterface(iid, object) }.ok()
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn LockServer(&self, lock: BOOL) -> Result<()> {
        if lock.as_bool() {
            dll_add_ref();
        } else {
            dll_release();
        }
        Ok(())
    }
}

pub(crate) fn free_global_objects() {
    CLASS_FACTORY_OBJECT.lock().unwrap().take();
}

#[unsafe(no_mangle)]
#[expect(nonstandard_style, reason = "DLL export")]
pub(crate) fn DllGetClassObject(
    clsid: *const GUID,
    iid: *const GUID,
    object: *mut *mut c_void,
) -> HRESULT {
    let lock = CLASS_FACTORY_OBJECT.lock().unwrap();
    let factory = lock.get_or_init(|| ClassFactory.into_static());

    let clsid = unsafe { clsid.as_ref() }.unwrap();
    let iid = unsafe { iid.as_ref() }.unwrap();

    if (iid == &IClassFactory::IID || iid == &IUnknown::IID) && clsid == &IME_CLSID {
        let factory = factory.as_interface::<IClassFactory>().to_owned();
        unsafe { object.write(std::mem::transmute::<IClassFactory, *mut c_void>(factory)) };
        dll_add_ref(); // class factory holds DLL ref count
        S_OK
    } else {
        unsafe { object.write(std::ptr::null_mut()) };
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[unsafe(no_mangle)]
#[expect(nonstandard_style, reason = "DLL export")]
pub(crate) fn DllCanUnloadNow() -> HRESULT {
    if DLL_REF_COUNT.load(Ordering::Relaxed) < 0 {
        S_OK
    } else {
        S_FALSE
    }
}
