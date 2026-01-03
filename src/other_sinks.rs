// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

impl ITfTextEditSink_Impl for Ime_Impl {
    /// Called by the system whenever anyone releases a write-access document lock
    // #[tracing::instrument(skip_all, ret, err)]
    fn OnEndEdit(
        &self,
        ctx: Ref<'_, ITfContext>,
        edit_cookie: u32,
        edit_record: Ref<'_, ITfEditRecord>,
    ) -> Result<()> {
        let edit_record = edit_record.ok()?;
        if unsafe { edit_record.GetSelectionStatus() }?.as_bool() {
            return Ok(());
        }

        let Some(range) = self
            .composition()
            .map(|c| unsafe { c.tf_composition.GetRange() })
        else {
            return Ok(());
        };

        let selection = utils::get_selection(edit_cookie, ctx.ok()?, TF_DEFAULT_SELECTION)?;

        if let Some(selection_range) = selection.range
            && !utils::is_range_covered(edit_cookie, &selection_range, &range?)
        {
            tracing::trace!("range clobber");
            self.finish_composition(ctx.as_ref(), )?;
        }

        Ok(())
    }
}

impl ITfThreadFocusSink_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn OnSetThreadFocus(&self) -> Result<()> {
        Ok(())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnKillThreadFocus(&self) -> Result<()> {
        Ok(())
    }
}

impl ITfThreadMgrEventSink_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn OnInitDocumentMgr(&self, _: Ref<'_, ITfDocumentMgr>) -> Result<()> {
        E_NOTIMPL.ok()
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnUninitDocumentMgr(&self, _: Ref<'_, ITfDocumentMgr>) -> Result<()> {
        E_NOTIMPL.ok()
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnSetFocus(
        &self,
        focus: Ref<'_, ITfDocumentMgr>,
        _prev_focus: Ref<'_, ITfDocumentMgr>,
    ) -> Result<()> {
        let focus = focus.as_ref();

        self.update_text_edit_sink_focus(focus)?;

        self.last_focused.replace(focus.cloned());

        Ok(())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnPushContext(&self, _: Ref<'_, ITfContext>) -> Result<()> {
        E_NOTIMPL.ok()
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn OnPopContext(&self, _: Ref<'_, ITfContext>) -> Result<()> {
        E_NOTIMPL.ok()
    }
}

impl Ime_Impl {
    /// Init a text edit sink on the topmost context of the document.
    /// Always release any previous sink.
    // #[tracing::instrument(skip_all, ret, err)]
    fn update_text_edit_sink_focus(&self, doc_mgr: Option<&ITfDocumentMgr>) -> Result<()> {
        let Some(mut state) = self.state_mut() else {
            return Ok(());
        };

        if let Some(ctx) = state.text_edit_ctx.take() {
            let source = ctx.cast::<ITfSource>()?;
            unsafe { source.UnadviseSink(state.text_edit_sink_cookie) }?;
            state.text_edit_sink_cookie = TF_INVALID_COOKIE;
        }

        let Some(doc_mgr) = doc_mgr else {
            return Ok(());
        };

        let ctx = match unsafe { doc_mgr.GetTop() } {
            Ok(ctx) => ctx,
            Err(e) if e.code() == S_OK => return Ok(()),
            Err(e) => return Err(e),
        };

        let source = ctx.cast::<ITfSource>()?;
        let cookie = unsafe { source.AdviseSink(&ITfTextEditSink::IID, self.as_interface()) }?;
        state.text_edit_sink_cookie = cookie;
        state.text_edit_ctx.replace(ctx);

        Ok(())
    }
}
