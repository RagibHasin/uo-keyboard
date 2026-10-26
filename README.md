# Ũõ (ঙ) Keyboard

A Windows Text Service Framework based IME for Bangla language implementing [Avro Phonetic](https://www.omicronlab.com/avro-keyboard.html) and [Khipro](https://khipro.khiproteam.com/) schemes.

Code is loosely based on the [sample IME from Windows classic samples](https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/IME/) and its partial [Rust port](https://github.com/saschanaz/ime-rs).

Avro Phonetic translation uses another fantastic open-source project [`okkhor`](https://github.com/gulshan/okkhor/).

Khipro `.mim` files are from [Khipro installer](https://github.com/KhiproTeam/khipro-installer), and are utilized through [`m17n-im`](https://github.com/RagibHasin/m17n-im/).

## How to use

1. Download and install `uo_keyboard_setup_*.exe` from the [latest](https://github.com/RagibHasin/uo-keyboard/releases) release.
2. Press <kbd>⊞ + Space</kbd> to cycle through IMEs.

## License

This project is licensed under Mozilla Public License 2.0, following the precedent of its inspirations.
