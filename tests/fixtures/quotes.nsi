OutFile "quotes.exe"

Section
	DetailPrint "installer"
	DetailPrint 'installer'
	DetailPrint `installer`

	DetailPrint "Installer with 'quote'"
	DetailPrint 'Installer with "quote"'
	DetailPrint `Installer with "quote"`

	DetailPrint "Installer with $\"quote$\""
	DetailPrint 'Installer with $\'quote$\''
	DetailPrint `Installer with $\"quote$\"`

	DetailPrint "She said $\"it's done$\""
	DetailPrint `She said $\"it's done$\"`

	DetailPrint "All $\"three$\" 'quote' `types`"
SectionEnd
