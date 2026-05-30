OutFile "quotes.exe"

Section
	OutFile "installer"
	OutFile 'installer'
	OutFile `installer`

	OutFile "Installer with 'quote'"
	OutFile 'Installer with "quote"'
	OutFile `Installer with "quote"`

	OutFile "Installer with $\"quote$\""
	OutFile 'Installer with $\'quote$\''
	OutFile `Installer with $\"quote$\"`

	OutFile "She said $\"it's done$\""
	OutFile `She said $\"it's done$\"`

	OutFile "All $\"three$\" 'quote' `types`"
SectionEnd
