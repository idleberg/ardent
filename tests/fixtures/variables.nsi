; Built-in variable, define and language string casing

!addincludedir "${nsisdir}\Include"

Name "Ardent"
OutFile "variables.exe"
InstallDir "$programfiles\Ardent"

Var CustomVar

Section "Main"
	setoutpath $instdir
	strcpy $r0 "$instdir\bin"
	strcpy $CustomVar "$instdir$temp"
	strcpy $1 "${nsisdir}"
	detailprint "$exedir\$exefile"
	detailprint "$appdata|$localappdata"
	detailprint "$%windir%\system32"
	detailprint "${MyOwnDefine}"
	detailprint "$myOwnVariable"
	detailprint "$(^completed) $(^name)"
	detailprint "$(MyOwnLangString)"
	detailprint "$$instdir is escaped"
	intop $r1 $r1 + 1
	writeregstr hklm "Software\Ardent" "Path" "$instdir"
	createshortcut "$smprograms\Ardent.lnk" "$instdir\ardent.exe"
SectionEnd
