def main [action: string] {
    let install_dir = 'C:\Program Files\UoKeyboardDev'
    let dll_filename = 'uo_keyboard.dll'
    let dll_src_path = $"target\\release\\($dll_filename)"
    let dll_dest_path = $"($install_dir)\\($dll_filename)"

    match $action {
        install => {
            ^cargo build --release

            if not ($dll_src_path | path exists) {
                error make { msg: $"Source DLL not found: ($dll_src_path)" }
            }

            ^gsudo.exe mkdir $install_dir
            ^gsudo.exe cp $dll_src_path $install_dir

            ^gsudo.exe regsvr32.exe $dll_dest_path /s

            if $env.LAST_EXIT_CODE != 0 {
                error make { msg: "DLL registration failed." }
            }

            print "Installation completed successfully."
        }

        uninstall => {
            if ($dll_dest_path | path exists) {
                ^gsudo.exe regsvr32.exe $dll_dest_path /s /u

                if $env.LAST_EXIT_CODE != 0 {
                    error make { msg: "DLL unregistration failed; directory was not deleted." }
                }
            }

            if ($install_dir | path exists) {
                ^gsudo.exe rm --recursive --force $install_dir
            }

            print "Uninstallation completed successfully."
        }

        _ => {
            error make {
                msg: $"Invalid argument: ($action). Usage: nu install.nu <install|uninstall>"
            }
        }
    }
}