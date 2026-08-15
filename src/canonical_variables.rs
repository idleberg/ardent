use std::collections::HashMap;
use std::sync::LazyLock;

/// Maps lowercased NSIS built-in variable names (without the leading `$`) to their
/// canonical casing.
///
/// Sources: `Source/build.cpp` (`m_UserVarNames`, `m_ShellConstants`), `Docs/src/var.but`.
///
/// Built-in variable names cannot be shadowed — `Var temp` fails to compile with
/// `variable "temp" already declared` — so rewriting these is always safe.
pub static BUILTIN_VARIABLES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
	let mut map = HashMap::new();

	// Registers $0-$9 and $R0-$R9
	const REGISTERS: [(&str, &str); 20] = [
		("0", "0"),
		("1", "1"),
		("2", "2"),
		("3", "3"),
		("4", "4"),
		("5", "5"),
		("6", "6"),
		("7", "7"),
		("8", "8"),
		("9", "9"),
		("r0", "R0"),
		("r1", "R1"),
		("r2", "R2"),
		("r3", "R3"),
		("r4", "R4"),
		("r5", "R5"),
		("r6", "R6"),
		("r7", "R7"),
		("r8", "R8"),
		("r9", "R9"),
	];
	map.extend(REGISTERS);

	// Named user variables
	map.extend([
		("cmdline", "CMDLINE"),
		("instdir", "INSTDIR"),
		("outdir", "OUTDIR"),
		("exedir", "EXEDIR"),
		("language", "LANGUAGE"),
		("temp", "TEMP"),
		("pluginsdir", "PLUGINSDIR"),
		("exepath", "EXEPATH"),
		("exefile", "EXEFILE"),
		("hwndparent", "HWNDPARENT"),
		("_click", "_CLICK"),
		("_outdir", "_OUTDIR"),
	]);

	// Shell constants
	map.extend([
		("windir", "WINDIR"),
		("sysdir", "SYSDIR"),
		("smprograms", "SMPROGRAMS"),
		("smstartup", "SMSTARTUP"),
		("desktop", "DESKTOP"),
		("startmenu", "STARTMENU"),
		("quicklaunch", "QUICKLAUNCH"),
		("documents", "DOCUMENTS"),
		("sendto", "SENDTO"),
		("recent", "RECENT"),
		("favorites", "FAVORITES"),
		("music", "MUSIC"),
		("pictures", "PICTURES"),
		("videos", "VIDEOS"),
		("nethood", "NETHOOD"),
		("fonts", "FONTS"),
		("templates", "TEMPLATES"),
		("appdata", "APPDATA"),
		("localappdata", "LOCALAPPDATA"),
		("printhood", "PRINTHOOD"),
		("internet_cache", "INTERNET_CACHE"),
		("cookies", "COOKIES"),
		("history", "HISTORY"),
		("profile", "PROFILE"),
		("admintools", "ADMINTOOLS"),
		("resources", "RESOURCES"),
		("resources_localized", "RESOURCES_LOCALIZED"),
		("cdburn_area", "CDBURN_AREA"),
	]);

	// Shell constants unaffected by SetShellVarContext
	map.extend([
		("userappdata", "USERAPPDATA"),
		("userlocalappdata", "USERLOCALAPPDATA"),
		("usertemplates", "USERTEMPLATES"),
		("userstartmenu", "USERSTARTMENU"),
		("usersmprograms", "USERSMPROGRAMS"),
		("userdesktop", "USERDESKTOP"),
		("commonlocalappdata", "COMMONLOCALAPPDATA"),
		("commonprogramdata", "COMMONPROGRAMDATA"),
		("commontemplates", "COMMONTEMPLATES"),
		("commonstartmenu", "COMMONSTARTMENU"),
		("commonsmprograms", "COMMONSMPROGRAMS"),
		("commondesktop", "COMMONDESKTOP"),
	]);

	// Registry-resolved constants
	map.extend([
		("programfiles", "PROGRAMFILES"),
		("programfiles32", "PROGRAMFILES32"),
		("programfiles64", "PROGRAMFILES64"),
		("commonfiles", "COMMONFILES"),
		("commonfiles32", "COMMONFILES32"),
		("commonfiles64", "COMMONFILES64"),
	]);

	map
});

/// Maps lowercased NSIS built-in defines (including the `${...}` delimiters) to their
/// canonical casing.
///
/// Sources: `Source/build.cpp` (`definedlist`), `Source/scriptpp.cpp`, `Source/script.cpp`.
pub static BUILTIN_DEFINES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
	HashMap::from([
		("${nsisdir}", "${NSISDIR}"),
		("${nsis_version}", "${NSIS_VERSION}"),
		("${nsis_packedversion}", "${NSIS_PACKEDVERSION}"),
		("${nsis_char_size}", "${NSIS_CHAR_SIZE}"),
		("${nsis_ptr_size}", "${NSIS_PTR_SIZE}"),
		("${nsis_max_strlen}", "${NSIS_MAX_STRLEN}"),
		// Standard predefines
		("${__counter__}", "${__COUNTER__}"),
		("${__date__}", "${__DATE__}"),
		("${__file__}", "${__FILE__}"),
		("${__filedir__}", "${__FILEDIR__}"),
		("${__function__}", "${__FUNCTION__}"),
		("${__global__}", "${__GLOBAL__}"),
		("${__line__}", "${__LINE__}"),
		("${__macro__}", "${__MACRO__}"),
		("${__pageex__}", "${__PAGEEX__}"),
		("${__section__}", "${__SECTION__}"),
		("${__time__}", "${__TIME__}"),
		("${__timestamp__}", "${__TIMESTAMP__}"),
		("${__uninstall__}", "${__UNINSTALL__}"),
	])
});

/// Maps lowercased NSIS built-in language string names (including the leading `^`, without
/// the `$(...)` delimiters) to their canonical casing.
///
/// Source: the `NLFRef` table in `Source/lang.cpp`.
pub static BUILTIN_LANGSTRINGS: LazyLock<HashMap<&'static str, &'static str>> =
	LazyLock::new(|| {
		HashMap::from([
			("^branding", "^Branding"),
			("^setupcaption", "^SetupCaption"),
			("^uninstallcaption", "^UninstallCaption"),
			("^licensesubcaption", "^LicenseSubCaption"),
			("^componentssubcaption", "^ComponentsSubCaption"),
			("^dirsubcaption", "^DirSubCaption"),
			("^installingsubcaption", "^InstallingSubCaption"),
			("^completedsubcaption", "^CompletedSubCaption"),
			("^uncomponentssubcaption", "^UnComponentsSubCaption"),
			("^undirsubcaption", "^UnDirSubCaption"),
			("^confirmsubcaption", "^ConfirmSubCaption"),
			("^uninstallingsubcaption", "^UninstallingSubCaption"),
			("^uncompletedsubcaption", "^UnCompletedSubCaption"),
			("^backbtn", "^BackBtn"),
			("^nextbtn", "^NextBtn"),
			("^agreebtn", "^AgreeBtn"),
			("^acceptbtn", "^AcceptBtn"),
			("^dontacceptbtn", "^DontAcceptBtn"),
			("^installbtn", "^InstallBtn"),
			("^uninstallbtn", "^UninstallBtn"),
			("^cancelbtn", "^CancelBtn"),
			("^closebtn", "^CloseBtn"),
			("^browsebtn", "^BrowseBtn"),
			("^showdetailsbtn", "^ShowDetailsBtn"),
			("^clicknext", "^ClickNext"),
			("^clickinstall", "^ClickInstall"),
			("^clickuninstall", "^ClickUninstall"),
			("^name", "^Name"),
			("^nameda", "^NameDA"),
			("^completed", "^Completed"),
			("^licensetext", "^LicenseText"),
			("^licensetextcb", "^LicenseTextCB"),
			("^licensetextrb", "^LicenseTextRB"),
			("^unlicensetext", "^UnLicenseText"),
			("^unlicensetextcb", "^UnLicenseTextCB"),
			("^unlicensetextrb", "^UnLicenseTextRB"),
			("^licensedata", "^LicenseData"),
			("^custom", "^Custom"),
			("^componentstext", "^ComponentsText"),
			("^componentssubtext1", "^ComponentsSubText1"),
			(
				"^componentssubtext2_noinsttypes",
				"^ComponentsSubText2_NoInstTypes",
			),
			("^componentssubtext2", "^ComponentsSubText2"),
			("^uncomponentstext", "^UnComponentsText"),
			("^uncomponentssubtext1", "^UnComponentsSubText1"),
			(
				"^uncomponentssubtext2_noinsttypes",
				"^UnComponentsSubText2_NoInstTypes",
			),
			("^uncomponentssubtext2", "^UnComponentsSubText2"),
			("^dirtext", "^DirText"),
			("^dirsubtext", "^DirSubText"),
			("^dirbrowsetext", "^DirBrowseText"),
			("^undirtext", "^UnDirText"),
			("^undirsubtext", "^UnDirSubText"),
			("^undirbrowsetext", "^UnDirBrowseText"),
			("^spaceavailable", "^SpaceAvailable"),
			("^spacerequired", "^SpaceRequired"),
			("^uninstallingtext", "^UninstallingText"),
			("^uninstallingsubtext", "^UninstallingSubText"),
			("^fileerror", "^FileError"),
			("^fileerror_noignore", "^FileError_NoIgnore"),
			("^cantwrite", "^CantWrite"),
			("^copyfailed", "^CopyFailed"),
			("^copyto", "^CopyTo"),
			("^registering", "^Registering"),
			("^unregistering", "^Unregistering"),
			("^symbolnotfound", "^SymbolNotFound"),
			("^couldnotload", "^CouldNotLoad"),
			("^createfolder", "^CreateFolder"),
			("^createshortcut", "^CreateShortcut"),
			("^createduninstaller", "^CreatedUninstaller"),
			("^delete", "^Delete"),
			("^deleteonreboot", "^DeleteOnReboot"),
			("^errorcreatingshortcut", "^ErrorCreatingShortcut"),
			("^errorcreating", "^ErrorCreating"),
			("^errordecompressing", "^ErrorDecompressing"),
			("^errorregistering", "^ErrorRegistering"),
			("^execshell", "^ExecShell"),
			("^exec", "^Exec"),
			("^extract", "^Extract"),
			("^errorwriting", "^ErrorWriting"),
			("^invalidopcode", "^InvalidOpcode"),
			("^noole", "^NoOLE"),
			("^outputfolder", "^OutputFolder"),
			("^removefolder", "^RemoveFolder"),
			("^renameonreboot", "^RenameOnReboot"),
			("^rename", "^Rename"),
			("^skipped", "^Skipped"),
			("^copydetails", "^CopyDetails"),
			("^loginstall", "^LogInstall"),
			("^byte", "^Byte"),
			("^kilo", "^Kilo"),
			("^mega", "^Mega"),
			("^giga", "^Giga"),
			("^font", "^Font"),
			("^fontsize", "^FontSize"),
			("^rtl", "^RTL"),
			("^language", "^Language"),
		])
	});
