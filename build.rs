// Copyright 2026 Muhammad Ragib Hasin
// SPDX-License-Identifier: MPL-2.0

fn main() {
    const IME_ICON_INDEX_AVRO: &str = "11";
    const IME_ICON_INDEX_KHIPRO_CLASSIC: &str = "12";
    const IME_ICON_INDEX_KHIPRO_DARK: &str = "13";

    winres::WindowsResource::new()
        .set_icon_with_id("resources/Avro.ico", IME_ICON_INDEX_AVRO)
        .set_icon_with_id("resources/KhiproClassic.ico", IME_ICON_INDEX_KHIPRO_CLASSIC)
        .set_icon_with_id("resources/KhiproDark.ico", IME_ICON_INDEX_KHIPRO_DARK)
        .compile()
        .unwrap();

    println!("cargo::rerun-if-changed=mim/bn-khipro-classic.mim");

    m17n_im::Parser::parse_from_path("mim/bn-khipro-classic.mim")
        .unwrap()
        .generate(
            &mut m17n_im::Preprocessor::default(),
            &mut std::fs::File::create("src/generated/bn-khipro-classic.rs").unwrap(),
        )
        .unwrap();

    println!("cargo::rerun-if-changed=mim/bn-khipro-dark.mim");

    m17n_im::Parser::parse_from_path("mim/bn-khipro-dark.mim")
        .unwrap()
        .generate(
            &mut m17n_im::Preprocessor::default(),
            &mut std::fs::File::create("src/generated/bn-khipro-dark.rs").unwrap(),
        )
        .unwrap();
}
