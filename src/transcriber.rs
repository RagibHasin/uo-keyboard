// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

use crate::*;

mod khipro_data {
    #[path = "../../generated/bn-khipro-classic.rs"]
    pub(super) mod classic;
    // #[path = "../../generated/bn-khipro-dark.rs"]
    // pub(super) mod dark;
}

pub(crate) enum Transcriber {
    Avro(okkhor::parser::Parser),
    Khipro,
}

impl std::fmt::Debug for Transcriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Avro(_) => f.debug_tuple("Avro").finish_non_exhaustive(),
            Self::Khipro => f.write_str("Khipro"),
        }
    }
}

impl Transcriber {
    pub(crate) fn new(profile: GUID) -> Self {
        match profile {
            globals::IME_PROFILE_AVRO => Self::Avro(okkhor::parser::Parser::new_phonetic()),
            globals::IME_PROFILE_KHIPRO => Self::Khipro,
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
            Transcriber::Khipro => {
                output.clear();
                let mut engine = khipro_data::classic::instantiate_engine();
                for word in raw_input.split_whitespace() {
                    output.push_str(&engine.convert(word));
                    output.push(' ');
                }
                output.pop();
            }
        }
    }

    pub(crate) fn adapt_char(&self, ch: u8) -> char {
        match self {
            Transcriber::Avro(_) => ch as _,
            Transcriber::Khipro => ch.to_ascii_lowercase() as _,
        }
    }

    pub(crate) fn dot_trailer(&self) -> u8 {
        match self {
            Transcriber::Avro(_) => b'`',
            Transcriber::Khipro => b'.',
        }
    }
}
