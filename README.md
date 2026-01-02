# Ũõ (ঙ) Keyboard

A Windows Text Service Framework based IME for Bangla language implementing [Avro Phonetic](https://www.omicronlab.com/avro-keyboard.html) scheme.

Code is loosely based on the [sample IME from Windows classic samples](https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/IME/) and its partial [Rust port](https://github.com/saschanaz/ime-rs).

Phonetic translation uses another fantastic open-source project [`rupantor`](https://github.com/OpenBangla/rupantor-rs).

## How to use

1. Download `uo_keyboard.dll` from the [latest](https://github.com/RagibHasin/uo-keyboard/releases) release.
2. Run `regsvr32 uo_keyboard.dll` from an elevated terminal.
3. Press <kbd>⊞ + Space</kbd> to cycle through IMEs.

## License

This project is licensed under Mozilla Public License 2.0, following the precedent of its inspirations.
