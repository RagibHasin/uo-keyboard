// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

pub(crate) enum Transcriber {
    Avro(okkhor::parser::Parser),
    Khipro(okkhor::khipro::KhiproPhonetic),
}

impl std::fmt::Debug for Transcriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Avro(_) => f.debug_tuple("Avro").finish_non_exhaustive(),
            Self::Khipro(_) => f.debug_tuple("Khipro").finish_non_exhaustive(),
        }
    }
}

impl Transcriber {
    pub(crate) fn new(profile: GUID) -> Self {
        match profile {
            globals::IME_PROFILE_AVRO => Self::Avro(okkhor::parser::Parser::new_phonetic()),
            globals::IME_PROFILE_KHIPRO => Self::Khipro(okkhor::khipro::KhiproPhonetic::new()),
            _ => panic!("Transcriber only supports Avro or Khipro profiles"),
        }
    }

    pub(crate) fn convert(&self, raw_input: &str) -> String {
        let mut output = String::with_capacity(64);
        self.convert_into(raw_input, &mut output);
        output
    }

    pub(crate) fn convert_into(&self, raw_input: &str, output: &mut String) {
        match self {
            Transcriber::Avro(scribe) => scribe.convert_into(raw_input, output),
            Transcriber::Khipro(scribe) => scribe.convert_into(raw_input, output),
        }
    }

    pub(crate) fn adapt_char(&self, ch: u8) -> char {
        match self {
            Transcriber::Avro(_) => ch as _,
            Transcriber::Khipro(_) => ch.to_ascii_lowercase() as _,
        }
    }

    pub(crate) fn dot_trailer(&self) -> u8 {
        match self {
            Transcriber::Avro(_) => b'`',
            Transcriber::Khipro(_) => b'.',
        }
    }
}
