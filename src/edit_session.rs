// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

#[derive(Debug)]
pub(crate) struct Composition {
    pub(crate) tf_composition: ITfComposition,
    ctx: ITfContext,
    input: String,
}

#[derive(Debug)]
struct EditSession {
    ime: ComObject<Ime>,
    ctx: ITfContext,
}

impl EditSession {
    fn new(ime: &Ime_Impl, ctx: &ITfContext) -> Self {
        EditSession {
            ime: ime.to_object(),
            ctx: ctx.clone(),
        }
    }

    #[tracing::instrument(skip_all, ret, err)]
    fn start_composition(&self, edit_cookie: u32) -> Result<()> {
        if self.ime.composition().is_some() {
            tracing::trace!("composition already exists");
            return Ok(());
        }

        let ias = self.ctx.cast::<ITfInsertAtSelection>()?;
        let insert_range =
            unsafe { ias.InsertTextAtSelection(edit_cookie, TF_IAS_QUERYONLY, &[]) }?;

        let ctx_composition = self.ctx.cast::<ITfContextComposition>()?;
        let composition = unsafe {
            ctx_composition.StartComposition(edit_cookie, &insert_range, self.ime.as_interface())
        }?;

        utils::set_selection(
            edit_cookie,
            &self.ctx,
            utils::TfSelection {
                range: Some(insert_range),
                style: TF_SELECTIONSTYLE {
                    ase: TF_AE_NONE,
                    fInterimChar: FALSE,
                },
            },
        )?;

        self.ime.state_mut().unwrap().composition = Some(Composition {
            tf_composition: composition,
            ctx: self.ctx.clone(),
            input: String::new(),
        });

        Ok(())
    }

    #[tracing::instrument(skip_all, ret, err)]
    fn terminate_composition(&self, edit_cookie: u32) -> Result<()> {
        let Some(composition) = self.ime.state_mut().unwrap().composition.take() else {
            tracing::trace!("composition doesn't exist");
            return Ok(());
        };

        let selection = utils::get_selection(edit_cookie, &self.ctx, TF_DEFAULT_SELECTION)
            .map_err(|e| Error::new(e.code(), "failed to get selection"))?;
        let selection_range = selection.range.as_ref().unwrap();

        unsafe { selection_range.Collapse(edit_cookie, TF_ANCHOR_END) }
            .map_err(|e| Error::new(e.code(), "failed to collapse selection"))?;
        utils::set_selection(edit_cookie, &self.ctx, selection)
            .map_err(|e| Error::new(e.code(), "failed to set selection"))?;

        match unsafe { composition.tf_composition.EndComposition(edit_cookie) } {
            Ok(()) => Ok(()),
            Err(e) if e.code() == E_UNEXPECTED => Ok(()),
            Err(e) => Err(Error::new(e.code(), "failed to end compoition")),
        }
    }

    #[tracing::instrument(skip_all, ret, err)]
    fn update_composition(&self, edit_cookie: u32) -> Result<()> {
        let composition = self.ime.composition().unwrap();

        let converted = self.ime.converter.convert(&composition.input);
        tracing::trace!(composition.input, converted);

        let encoded = converted.encode_utf16().collect::<Vec<_>>();

        let range = unsafe { composition.tf_composition.GetRange() }?;

        unsafe { range.SetText(edit_cookie, 0, &encoded) }?;
        tracing::trace!("set text");

        if !encoded.is_empty() {
            self.set_prop(
                edit_cookie,
                &range,
                GUID_PROP_LANGID,
                globals::IME_LANGID as i32,
            )?;
            tracing::trace!("set composition lang");
        }

        // update the selection, we'll make it an insertion point just past the inserted text.
        let selection_range = unsafe { range.Clone() }?;
        unsafe { selection_range.Collapse(edit_cookie, TF_ANCHOR_END) }?;

        utils::set_selection(
            edit_cookie,
            &self.ctx,
            utils::TfSelection {
                range: Some(selection_range),
                style: TF_SELECTIONSTYLE {
                    ase: TF_AE_NONE,
                    fInterimChar: FALSE,
                },
            },
        )?;

        Ok(())
    }

    // #[tracing::instrument(skip_all, ret, err)]
    fn set_prop(&self, edit_cookie: u32, range: &ITfRange, prop: GUID, value: i32) -> Result<()> {
        let language_prop = unsafe { self.ctx.GetProperty(&prop) }?;
        let var = windows::Win32::System::Variant::VARIANT::from(value);
        unsafe { language_prop.SetValue(edit_cookie, range, &var) }
    }
}

#[implement(ITfEditSession)]
#[derive(Debug)]
struct AddSingleEditSession {
    base: EditSession,
    ch: u8,
}

impl ITfEditSession_Impl for AddSingleEditSession_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn DoEditSession(&self, edit_cookie: u32) -> Result<()> {
        let ctx = &self.base.ctx;

        let input = std::str::from_utf8(std::slice::from_ref(&self.ch)).unwrap();
        let converted = self.base.ime.converter.convert(input);
        tracing::trace!(input, converted);

        let encoded = converted.encode_utf16().collect::<Vec<_>>();

        let selection = utils::get_selection(edit_cookie, ctx, TF_DEFAULT_SELECTION)?;
        let selection_range = selection.range.as_ref().unwrap();

        unsafe { selection_range.SetText(edit_cookie, 0, &encoded) }?;
        unsafe { selection_range.Collapse(edit_cookie, TF_ANCHOR_END) }?;

        utils::set_selection(edit_cookie, ctx, selection)
    }
}

#[implement(ITfEditSession)]
#[derive(Debug)]
struct AppendEditSession {
    base: EditSession,
    ch: u8,
}

impl ITfEditSession_Impl for AppendEditSession_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn DoEditSession(&self, edit_cookie: u32) -> Result<()> {
        self.base.start_composition(edit_cookie)?;
        self.base
            .ime
            .composition_mut()
            .unwrap()
            .input
            .push(self.ch as _);

        self.base.update_composition(edit_cookie)
    }
}

#[implement(ITfEditSession)]
#[derive(Debug)]
struct PopCharEditSession {
    base: EditSession,
}

impl ITfEditSession_Impl for PopCharEditSession_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn DoEditSession(&self, edit_cookie: u32) -> Result<()> {
        self.base.ime.composition_mut().unwrap().input.pop();

        self.base.update_composition(edit_cookie)?;

        let is_input_empty = self.base.ime.composition().unwrap().input.is_empty();
        if is_input_empty {
            self.base.terminate_composition(edit_cookie)?;
        }

        Ok(())
    }
}

#[implement(ITfEditSession)]
#[derive(Debug)]
struct FinishEditSession {
    base: EditSession,
}

impl ITfEditSession_Impl for FinishEditSession_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn DoEditSession(&self, edit_cookie: u32) -> Result<()> {
        self.base.terminate_composition(edit_cookie)
    }
}

impl Ime_Impl {
    fn request_edit_session(
        &self,
        ctx: &ITfContext,
        edit_session: impl ComObjectInner<Outer: ComObjectInterface<ITfEditSession>>,
    ) -> Result<()> {
        let edit_session = edit_session.into_object();
        unsafe {
            ctx.RequestEditSession(
                self.client_id(),
                edit_session.as_interface(),
                TF_ES_SYNC | TF_ES_READWRITE,
            )
        }
        .map(|_| ())
    }

    #[tracing::instrument(skip_all, ret, err)]
    pub(crate) fn add_single_char(&self, ctx: &ITfContext, ch: u8) -> Result<()> {
        self.request_edit_session(
            ctx,
            AddSingleEditSession {
                base: EditSession::new(self, ctx),
                ch,
            },
        )
    }

    #[tracing::instrument(skip_all, ret, err)]
    pub(crate) fn append_char_to_composition(&self, ctx: &ITfContext, ch: u8) -> Result<()> {
        self.request_edit_session(
            ctx,
            AppendEditSession {
                base: EditSession::new(self, ctx),
                ch,
            },
        )
    }

    #[tracing::instrument(skip_all, ret, err)]
    pub(crate) fn pop_char_from_composition(&self, ctx: &ITfContext) -> Result<()> {
        self.request_edit_session(
            ctx,
            PopCharEditSession {
                base: EditSession::new(self, ctx),
            },
        )
    }

    #[tracing::instrument(skip_all, ret, err)]
    pub(crate) fn finish_composition(&self, ctx: Option<&ITfContext>) -> Result<()> {
        let Some(ctx) = ctx
            .cloned()
            .or_else(|| self.composition().map(|c| c.ctx.clone()))
        else {
            return Ok(());
        };

        self.request_edit_session(
            &ctx,
            FinishEditSession {
                base: EditSession::new(self, &ctx),
            },
        )
    }
}

impl ITfCompositionSink_Impl for Ime_Impl {
    // #[tracing::instrument(skip_all, ret, err)]
    fn OnCompositionTerminated(&self, _: u32, _: Ref<'_, ITfComposition>) -> Result<()> {
        tracing::trace!("composition termination");
        self.finish_composition(None)
    }
}
