fn main() {
    const IME_ICON_INDEX: &str = "12";

    winres::WindowsResource::new()
        .set_icon_with_id("resources/IME.ico", IME_ICON_INDEX)
        .compile()
        .unwrap();
}
