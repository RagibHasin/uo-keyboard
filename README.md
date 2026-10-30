<div align="center">

![](resources/IME.ico)

# Ũõ Keyboard

</div>

A Windows Text Service Framework based IME for Bangla language implementing [Avro Phonetic](https://www.omicronlab.com/avro-keyboard.html) and [Khipro](https://khipro.khiproteam.com/) schemes.

Its code is loosely based on the [sample IME from Windows classic samples](https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/IME/) and its partial [Rust port](https://github.com/saschanaz/ime-rs).

Avro Phonetic translation uses another fantastic open-source project [`okkhor`](https://github.com/gulshan/okkhor/).

Khipro `.mim` files are from [Khipro installer](https://github.com/KhiproTeam/khipro-installer), and are utilized through [`m17n-im`](https://github.com/RagibHasin/m17n-im/).

This IME bundles [v37.0.3](https://github.com/rank-coder/khipro-m17n/releases/tag/clsc-v37.0.3) of Khipro Classic and [v2.0.2](https://github.com/KhiproTeam/khipro-dark-m17n/releases/tag/drk-v2.0.2) of Khipro Dark.

## How to use

1. Download and install `uo_keyboard_setup.exe` from the [latest](https://github.com/RagibHasin/uo-keyboard/releases) release.
2. Press <kbd>⊞ + Space</kbd> to cycle through IMEs.

## License

This project is licensed under Mozilla Public License 2.0, following the precedent of its inspirations.
