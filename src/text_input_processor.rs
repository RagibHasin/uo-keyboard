// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

impl Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn activate(
        &self,
        thread_mgr: Ref<'_, ITfThreadMgr>,
        client_id: u32,
    ) -> Result<ActiveImeState> {
        let thread_mgr = thread_mgr.ok()?.clone();
        let source = thread_mgr.cast::<ITfSource>()?;

        let thread_mgr_event_sink_cookie =
            unsafe { source.AdviseSink(&ITfThreadMgrEventSink::IID, self.as_interface()) }?;

        let text_edit_ctx = unsafe { thread_mgr.GetFocus()?.GetTop() }?;
        let text_edit_source = text_edit_ctx.cast::<ITfSource>()?;
        let text_edit_sink_cookie =
            unsafe { text_edit_source.AdviseSink(&ITfTextEditSink::IID, self.as_interface()) }?;

        let keystroke_mgr = thread_mgr.cast::<ITfKeystrokeMgr>()?;
        unsafe { keystroke_mgr.AdviseKeyEventSink(client_id, self.as_interface(), true) }?;

        let thread_focus_sink_cookie =
            unsafe { source.AdviseSink(&ITfThreadFocusSink::IID, self.as_interface()) }?;

        let single_source = thread_mgr.cast::<ITfSourceSingle>()?;
        unsafe {
            single_source.AdviseSingleSink(
                client_id,
                &ITfFunctionProvider::IID,
                self.as_interface(),
            )
        }?;

        Ok(ActiveImeState {
            thread_mgr,
            client_id,
            thread_mgr_event_sink_cookie,
            text_edit_ctx: Some(text_edit_ctx),
            text_edit_sink_cookie,
            thread_focus_sink_cookie,
            composition: None,
        })
    }
}

impl ActiveImeState {
    // #[tracing::instrument(skip_all, ret, err)]
    fn destroy(self) -> Result<()> {
        let Self {
            thread_mgr,
            client_id,
            thread_mgr_event_sink_cookie,
            thread_focus_sink_cookie,
            ..
        } = self;

        let single_source = thread_mgr.cast::<ITfSourceSingle>()?;
        unsafe { single_source.UnadviseSingleSink(client_id, &ITfFunctionProvider::IID) }?;

        let source = thread_mgr.cast::<ITfSource>()?;

        unsafe { source.UnadviseSink(thread_focus_sink_cookie) }?;

        let keystroke_mgr = thread_mgr.cast::<ITfKeystrokeMgr>()?;
        unsafe { keystroke_mgr.UnadviseKeyEventSink(client_id) }?;

        unsafe { source.UnadviseSink(thread_mgr_event_sink_cookie) }?;

        Ok(())
    }
}

impl ITfTextInputProcessor_Impl for Ime_Impl {
    fn Activate(&self, _: Ref<'_, ITfThreadMgr>, _: u32) -> Result<()> {
        E_NOTIMPL.ok()
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn Deactivate(&self) -> Result<()> {
        tracing::trace!("deactivate ime");
        self.finish_composition(None)?;

        if let Some(state) = self.state.take() {
            state.destroy()?;
        }

        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn ActivateEx(&self, thread_mgr: Ref<'_, ITfThreadMgr>, client_id: u32, _: u32) -> Result<()> {
        if let Ok(state) = self.activate(thread_mgr, client_id) {
            self.state.replace(Some(state));
            Ok(())
        } else {
            Err(if let Err(e) = self.Deactivate() {
                e
            } else {
                E_FAIL.into()
            })
        }
    }
}
