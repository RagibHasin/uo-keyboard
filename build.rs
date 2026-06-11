fn main() {
    const IME_ICON_INDEX_AVRO: &str = "11";
    const IME_ICON_INDEX_KHIPRO: &str = "12";

    winres::WindowsResource::new()
        .set_icon_with_id("resources/Avro.ico", IME_ICON_INDEX_AVRO)
        .set_icon_with_id("resources/Khipro.ico", IME_ICON_INDEX_KHIPRO)
        .compile()
        .unwrap();
}
