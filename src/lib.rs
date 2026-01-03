// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use std::cell::{self, RefCell};

use windows::Win32::{Foundation::*, UI::TextServices::*};
use windows::core::*;

mod utils;

mod globals;

mod dll;
mod factory;
mod registration;

mod compartment;
mod edit_session;
mod function_provider;
mod key_class;
mod key_event_sink;
mod other_sinks;
mod text_input_processor;

#[implement(
    ITfTextInputProcessorEx,
    ITfThreadMgrEventSink,
    ITfTextEditSink,
    ITfKeyEventSink,
    ITfCompositionSink,
    ITfThreadFocusSink,
    ITfFunctionProvider,
    ITfFunction,
    ITfFnGetPreferredTouchKeyboardLayout
)]
struct Ime {
    state: RefCell<Option<ActiveImeState>>,
    last_focused: RefCell<Option<ITfDocumentMgr>>,

    converter: rupantor::avro::AvroPhonetic,
}

#[derive(Debug)]
struct ActiveImeState {
    thread_mgr: ITfThreadMgr,
    client_id: u32,

    thread_mgr_event_sink_cookie: u32,

    text_edit_ctx: Option<ITfContext>,
    text_edit_sink_cookie: u32,

    thread_focus_sink_cookie: u32,

    composition: Option<edit_session::Composition>,
}

impl Ime {
    // #[tracing::instrument(skip_all, ret, err)]
    fn new() -> Result<Self> {
        factory::dll_add_ref();

        Ok(Ime {
            state: None.into(),
            last_focused: None.into(),

            converter: rupantor::avro::AvroPhonetic::new(),
        })
    }

    // #[tracing::instrument]
    fn state(&self) -> Option<cell::Ref<'_, ActiveImeState>> {
        cell::Ref::filter_map(self.state.borrow(), Option::as_ref).ok()
    }

    // #[tracing::instrument]
    fn state_mut(&self) -> Option<cell::RefMut<'_, ActiveImeState>> {
        cell::RefMut::filter_map(self.state.borrow_mut(), Option::as_mut).ok()
    }

    // #[tracing::instrument]
    fn composition(&self) -> Option<cell::Ref<'_, edit_session::Composition>> {
        self.state()
            .and_then(|s| cell::Ref::filter_map(s, |s| s.composition.as_ref()).ok())
    }

    // #[tracing::instrument]
    fn composition_mut(&self) -> Option<cell::RefMut<'_, edit_session::Composition>> {
        self.state_mut()
            .and_then(|s| cell::RefMut::filter_map(s, |s| s.composition.as_mut()).ok())
    }
}

impl Drop for Ime {
    fn drop(&mut self) {
        factory::dll_release();
    }
}

impl core::fmt::Debug for Ime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ime")
            .field("state", &self.state)
            .field("last_focused", &self.last_focused)
            .field("converter", &"<converter>")
            .finish()
    }
}
