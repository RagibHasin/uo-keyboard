// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use windows::Win32::{
    Foundation::{HMODULE, MAX_PATH},
    System::LibraryLoader::GetModuleFileNameW,
    UI::Input::KeyboardAndMouse::HKL,
};

use crate::*;

fn get_module_file_name(dll_instance_handle: HMODULE) -> Vec<u16> {
    let mut file_name = vec![0u16; MAX_PATH as usize];
    let file_name_len = unsafe { GetModuleFileNameW(Some(dll_instance_handle), &mut file_name) };
    file_name.drain(file_name_len as usize..);
    file_name
}

pub(crate) fn register_profile(dll_instance_handle: HMODULE) -> Result<()> {
    let profile_manager = utils::create_instance_inproc::<ITfInputProcessorProfileMgr>(
        &CLSID_TF_InputProcessorProfiles,
    )?;

    let icon_file_name = get_module_file_name(dll_instance_handle);
    let description = globals::IME_DESCRIPTION.encode_utf16().collect::<Vec<_>>();

    unsafe {
        profile_manager.RegisterProfile(
            &globals::IME_CLSID,
            globals::IME_LANGID,
            &globals::IME_PROFILE,
            &description,
            &icon_file_name,
            globals::IME_ICON_INDEX,
            HKL::default(),
            0,
            true,
            0,
        )
    }?;

    Ok(())
}

pub(crate) fn unregister_profile() -> Result<()> {
    let profile_manager = utils::create_instance_inproc::<ITfInputProcessorProfileMgr>(
        &CLSID_TF_InputProcessorProfiles,
    )?;

    unsafe {
        profile_manager.UnregisterProfile(
            &globals::IME_CLSID,
            globals::IME_LANGID,
            &globals::IME_PROFILE,
            0,
        )
    }?;

    Ok(())
}

const SUPPORT_CATEGORIES: [GUID; 8] = [
    GUID_TFCAT_TIP_KEYBOARD,
    GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
    GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    GUID_TFCAT_TIPCAP_SECUREMODE,
    GUID_TFCAT_TIPCAP_COMLESS,
    GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
];

pub(crate) fn register_categories() -> windows::core::Result<()> {
    let mgr = utils::create_instance_inproc::<ITfCategoryMgr>(&CLSID_TF_CategoryMgr)?;

    for guid in SUPPORT_CATEGORIES {
        unsafe { mgr.RegisterCategory(&globals::IME_CLSID, &guid, &globals::IME_CLSID) }?;
    }

    Ok(())
}

pub(crate) fn unregister_categories() -> windows::core::Result<()> {
    let mgr = utils::create_instance_inproc::<ITfCategoryMgr>(&CLSID_TF_CategoryMgr)?;

    for guid in SUPPORT_CATEGORIES {
        unsafe { mgr.UnregisterCategory(&globals::IME_CLSID, &guid, &globals::IME_CLSID) }?;
    }

    Ok(())
}

fn ime_key() -> String {
    format!("CLSID\\{{{:?}}}", globals::IME_CLSID)
}

pub(crate) fn register_server(dll_instance_handle: HMODULE) -> Result<()> {
    let key = windows_registry::CLASSES_ROOT.create(ime_key())?;
    key.set_string("", globals::IME_DESCRIPTION)?;

    let module_file_name = String::from_utf16(&get_module_file_name(dll_instance_handle)).unwrap();
    let inproc_key = key.create("InProcServer32")?;
    inproc_key.set_string("", module_file_name)?;
    inproc_key.set_string("ThreadingModel", "Apartment")?;

    Ok(())
}

pub(crate) fn unregister_server() -> Result<()> {
    windows_registry::CLASSES_ROOT.remove_tree(ime_key())
}
