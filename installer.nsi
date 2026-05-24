!include "MUI2.nsh"
!include "x64.nsh"
!include "Library.nsh"

!ifndef VERSION
	!error "VERSION must be defined. Use /DVERSION=<version>"
!endif

Name "Uo Keyboard"
OutFile "uo_keyboard_setup.exe"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "Uo Keyboard"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "FileDescription" "Uo Keyboard Installer"
VIAddVersionKey "LegalCopyright" "© 2026 Muhammad Ragib Hasin"

; --- MARK: MUI Settings
!define MUI_ABORTWARNING
!define MUI_ICON "resources/IME.ico"
!define MUI_UNICON "resources/IME.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

; --- MARK: INSTALL
Section "Install" SEC_MAIN

	SetOutPath $INSTDIR

	!insertmacro InstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "target\i686-pc-windows-msvc\release\uo_keyboard.dll" "$INSTDIR\uo_keyboard_x86.dll" "$INSTDIR"

	IfErrors 0 +2
	Abort "Failed to register x86 IME DLL."

	File "/oname=uo_keyboard_x86.pdb" "target\i686-pc-windows-msvc\release\uo_keyboard.pdb"

	${If} ${RunningX64}

		!define LIBRARY_X64
		!insertmacro InstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "target\release\uo_keyboard.dll" "$INSTDIR\uo_keyboard_amd64.dll" "$INSTDIR"

		IfErrors 0 +4
		!undef LIBRARY_X64
		!insertmacro UnInstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "target\i686-pc-windows-msvc\release\uo_keyboard.dll"
		Abort "Failed to register amd64 IME DLL."

		File "/oname=uo_keyboard_amd64.pdb" "target\release\uo_keyboard.pdb"

	${EndIf}

	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "DisplayName" "Uo Keyboard"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "DisplayVersion" "${VERSION}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "Publisher" "Your Name"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "UninstallString" '"$INSTDIR\uo_keyboard_uninstall.exe"'
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "NoModify" 1
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard" "NoRepair" 1

	WriteUninstaller "$INSTDIR\uo_keyboard_uninstall.exe"

SectionEnd

; --- MARK: UNINSTALL
Section "Uninstall"

	SetOutPath $INSTDIR

	!insertmacro UnInstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "uo_keyboard_x86.dll"

	IfErrors 0 +2
	Abort "Failed to unregister x86 IME DLL."

	Delete "uo_keyboard_x86.pdb"

	${If} ${RunningX64}

		!define LIBRARY_X64

		!insertmacro UnInstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "uo_keyboard_amd64.dll"

		IfErrors 0 +4
		!undef LIBRARY_X64
		!insertmacro UnInstallLib REGDLL NOTSHARED REBOOT_NOTPROTECTED "uo_keyboard_x86.dll"
		Abort "Failed to unregister amd64 IME DLL."

		Delete "uo_keyboard_amd64.pdb"

	${EndIf}

	Delete "$INSTDIR\uo_keyboard_uninstall.exe"
	DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\uo_keyboard"

SectionEnd
