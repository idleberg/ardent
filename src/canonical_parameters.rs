use std::collections::HashMap;
use std::sync::LazyLock;

/// Maps lowercased global parameters (e.g. `/silent`) to their canonical casing.
pub static GLOBAL_PARAMETERS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
	HashMap::from([
		("/silent", "/SILENT"),
		("/filesonly", "/FILESONLY"),
		("/rebootok", "/REBOOTOK"),
		("/short", "/SHORT"),
		("/sd", "/SD"),
		("/branding", "/BRANDING"),
		("/final", "/FINAL"),
		("/solid", "/SOLID"),
		("/global", "/GLOBAL"),
		("/bom", "/BOM"),
		("/italic", "/ITALIC"),
		("/underline", "/UNDERLINE"),
		("/strike", "/STRIKE"),
		("/enablecancel", "/ENABLECANCEL"),
		("/overwrite", "/OVERWRITE"),
		("/replace", "/REPLACE"),
		("/noerrors", "/NOERRORS"),
		("/regedit5", "/REGEDIT5"),
		("/exeresource", "/EXERESOURCE"),
		("/stringid", "/STRINGID"),
		("/resizetofit", "/RESIZETOFIT"),
		("/resizetofitwidth", "/RESIZETOFITWIDTH"),
		("/resizetofitheight", "/RESIZETOFITHEIGHT"),
		("/trimleft", "/TRIMLEFT"),
		("/trimright", "/TRIMRIGHT"),
		("/trimcenter", "/TRIMCENTER"),
		("/windows", "/windows"),
		("/nonfatal", "/NONFATAL"),
		("/nocustom", "/NOCUSTOM"),
		("/uninstnocustom", "/UNINSTNOCUSTOM"),
		("/componentsonlyoncustom", "/COMPONENTSONLYONCUSTOM"),
		(
			"/uninstcomponentsonlyoncustom",
			"/UNINSTCOMPONENTSONLYONCUSTOM",
		),
		("/fileexists", "/FILEEXISTS"),
		("/rawnl", "/RAWNL"),
		("/productversion", "/ProductVersion"),
		("/noworkingdir", "/NoWorkingDir"),
		("/r", "/r"),
		("/a", "/a"),
		("/e", "/e"),
		("/o", "/o"),
		("/x", "/x"),
		("/ifempty", "/ifempty"),
		("/ifnosubkeys", "/ifnosubkeys"),
		("/ifnovalues", "/ifnovalues"),
		("/nounload", "/nounload"),
		("/plugin", "/plugin"),
		("/ifndef", "/ifndef"),
		("/redef", "/redef"),
		("/date", "/date"),
		("/utcdate", "/utcdate"),
		("/file", "/file"),
		("/intfmt", "/intfmt"),
		("/math", "/math"),
		("/ignorecase", "/ignorecase"),
		("/packed", "/packed"),
		("/target", "/target"),
	])
});

/// Maps lowercased global parameter prefixes (e.g. `/lang=`) to their canonical casing.
pub static GLOBAL_PARAMETER_PREFIXES: LazyLock<HashMap<&'static str, &'static str>> =
	LazyLock::new(|| {
		HashMap::from([
			("/lang=", "/LANG="),
			("/timeout=", "/TIMEOUT="),
			("/charset=", "/CHARSET="),
			("/imgid=", "/IMGID="),
			("/customstring=", "/CUSTOMSTRING="),
			("/uninstcustomstring=", "/UNINSTCUSTOMSTRING="),
			("/oname=", "/oname="),
		])
	});

/// Maps lowercased instruction names to their per-instruction parameter casing tables.
pub static INSTRUCTION_PARAMETERS: LazyLock<
	HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = LazyLock::new(|| {
	let mut builder: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

	let mut register = |instructions: &[&'static str], params: &[(&'static str, &'static str)]| {
		for &instr in instructions {
			let map = builder.entry(instr).or_default();
			for &(k, v) in params {
				map.insert(k, v);
			}
		}
	};

	// Boolean values
	register(
		&[
			"allowrootdirinstall",
			"allowskipfiles",
			"autoclosewindow",
			"crccheck",
			"manifestdpiaware",
			"manifestdisablewindowfiltering",
			"manifestgdiscaling",
			"manifestlongpathaware",
			"setautoclose",
			"setdatablockoptimize",
			"setdatesave",
			"setpluginunload",
			"unicode",
			"windowicon",
		],
		&[("true", "true"), ("false", "false")],
	);

	register(
		&[
			"crccheck",
			"logset",
			"lockwindow",
			"setcompress",
			"setdatesave",
			"setdatablockoptimize",
			"setoverwrite",
			"windowicon",
			"xpstyle",
			"licenseforceselection",
		],
		&[("on", "on"), ("off", "off")],
	);

	// FileOpen modes
	register(&["fileopen"], &[("r", "r"), ("w", "w"), ("a", "a")]);

	// Compression algorithms
	register(
		&["setcompressor"],
		&[("zlib", "zlib"), ("bzip2", "bzip2"), ("lzma", "lzma")],
	);

	// CPU targets
	register(&["target"], &[("x86", "x86"), ("amd64", "amd64")]);

	// SetOverwrite modes
	register(
		&["setoverwrite"],
		&[
			("try", "try"),
			("ifnewer", "ifnewer"),
			("ifdiff", "ifdiff"),
			("lastused", "lastused"),
		],
	);

	// SetCompress modes
	register(&["setcompress"], &[("auto", "auto"), ("force", "force")]);

	// SilentInstall
	register(
		&["silentinstall"],
		&[
			("normal", "normal"),
			("silent", "silent"),
			("silentlog", "silentlog"),
		],
	);

	// SilentUnInstall / SetSilent
	register(
		&["silentuninstall", "setsilent"],
		&[("normal", "normal"), ("silent", "silent")],
	);

	// ShowInstDetails / ShowUninstDetails
	register(
		&["showinstdetails", "showuninstdetails"],
		&[
			("hide", "hide"),
			("show", "show"),
			("nevershow", "nevershow"),
		],
	);

	// SetDetailsView / DirShow
	register(
		&["setdetailsview", "dirshow"],
		&[("show", "show"), ("hide", "hide")],
	);

	// SetDetailsPrint
	register(
		&["setdetailsprint"],
		&[
			("listonly", "listonly"),
			("textonly", "textonly"),
			("both", "both"),
			("none", "none"),
			("lastused", "lastused"),
		],
	);

	// RequestExecutionLevel
	register(
		&["requestexecutionlevel"],
		&[
			("none", "none"),
			("user", "user"),
			("highest", "highest"),
			("admin", "admin"),
		],
	);

	// AddBrandingImage
	register(
		&["addbrandingimage"],
		&[
			("top", "top"),
			("left", "left"),
			("bottom", "bottom"),
			("right", "right"),
		],
	);

	// InstProgressFlags
	register(
		&["instprogressflags"],
		&[("smooth", "smooth"), ("colored", "colored")],
	);

	// LicenseForceSelection
	register(
		&["licenseforceselection"],
		&[("checkbox", "checkbox"), ("radiobuttons", "radiobuttons")],
	);

	// SetShellVarContext
	register(
		&["setshellvarcontext"],
		&[("all", "all"), ("current", "current")],
	);

	// DirVerify
	register(&["dirverify"], &[("auto", "auto"), ("leave", "leave")]);

	// ExecShell / ExecShellWait
	register(
		&["execshell", "execshellwait"],
		&[("open", "open"), ("print", "print")],
	);

	// Page / UninstPage
	register(
		&["page", "uninstpage"],
		&[
			("custom", "custom"),
			("license", "license"),
			("components", "components"),
			("directory", "directory"),
			("instfiles", "instfiles"),
			("uninstconfirm", "uninstConfirm"),
		],
	);

	// SetCtlColors
	register(&["setctlcolors"], &[("transparent", "transparent")]);

	// LockWindow
	register(&["lockwindow"], &[("on", "on"), ("off", "off")]);

	// SetRegView
	register(&["setregview"], &[("default", "default")]);

	// Registry root keys
	register(
		&[
			"deleteregkey",
			"deleteregvalue",
			"enumregkey",
			"enumregvalue",
			"installdirregkey",
			"readregdword",
			"readregstr",
			"writeregbin",
			"writeregdword",
			"writeregexpandstr",
			"writeregmultistr",
			"writeregnone",
			"writeregstr",
		],
		&[
			("hkcr", "HKCR"),
			("hkcr32", "HKCR32"),
			("hkcr64", "HKCR64"),
			("hklm", "HKLM"),
			("hklm32", "HKLM32"),
			("hklm64", "HKLM64"),
			("hkcu", "HKCU"),
			("hkcu32", "HKCU32"),
			("hkcu64", "HKCU64"),
			("hku", "HKU"),
			("hkcc", "HKCC"),
			("hkdd", "HKDD"),
			("hkpd", "HKPD"),
			("shctx", "SHCTX"),
		],
	);

	// MessageBox flags and return values
	register(
		&["messagebox"],
		&[
			("mb_ok", "MB_OK"),
			("mb_okcancel", "MB_OKCANCEL"),
			("mb_abortretryignore", "MB_ABORTRETRYIGNORE"),
			("mb_retrycancel", "MB_RETRYCANCEL"),
			("mb_yesno", "MB_YESNO"),
			("mb_yesnocancel", "MB_YESNOCANCEL"),
			("mb_iconexclamation", "MB_ICONEXCLAMATION"),
			("mb_iconinformation", "MB_ICONINFORMATION"),
			("mb_iconquestion", "MB_ICONQUESTION"),
			("mb_iconstop", "MB_ICONSTOP"),
			("mb_usericon", "MB_USERICON"),
			("mb_topmost", "MB_TOPMOST"),
			("mb_setforeground", "MB_SETFOREGROUND"),
			("mb_right", "MB_RIGHT"),
			("mb_defbutton1", "MB_DEFBUTTON1"),
			("mb_defbutton2", "MB_DEFBUTTON2"),
			("mb_defbutton3", "MB_DEFBUTTON3"),
			("mb_defbutton4", "MB_DEFBUTTON4"),
			("idok", "IDOK"),
			("idcancel", "IDCANCEL"),
			("idyes", "IDYES"),
			("idno", "IDNO"),
			("idabort", "IDABORT"),
			("idretry", "IDRETRY"),
			("idignore", "IDIGNORE"),
		],
	);

	// ShowWindow constants
	register(
		&["createshortcut", "showwindow"],
		&[
			("sw_shownormal", "SW_SHOWNORMAL"),
			("sw_showmaximized", "SW_SHOWMAXIMIZED"),
			("sw_showminimized", "SW_SHOWMINIMIZED"),
			("sw_hide", "SW_HIDE"),
			("sw_show", "SW_SHOW"),
		],
	);

	// Hotkey modifiers
	register(
		&["createshortcut"],
		&[
			("alt", "ALT"),
			("control", "CONTROL"),
			("ext", "EXT"),
			("shift", "SHIFT"),
		],
	);

	// File attributes
	register(
		&["setfileattributes"],
		&[
			("archive", "ARCHIVE"),
			("hidden", "HIDDEN"),
			("offline", "OFFLINE"),
			("readonly", "READONLY"),
			("system", "SYSTEM"),
			("temporary", "TEMPORARY"),
		],
	);

	// FileSeek modes
	register(
		&["fileseek"],
		&[("set", "SET"), ("cur", "CUR"), ("end", "END")],
	);

	// GetWinVer fields
	register(
		&["getwinver"],
		&[
			("major", "MAJOR"),
			("minor", "MINOR"),
			("build", "BUILD"),
			("servicepack", "SERVICEPACK"),
		],
	);

	// ManifestSupportedOS values
	register(
		&["manifestsupportedos"],
		&[
			("winvista", "WinVista"),
			("win7", "Win7"),
			("win8", "Win8"),
			("win8.1", "Win8.1"),
			("win10", "Win10"),
		],
	);

	// ChangeUI dialog identifiers
	register(&["changeui"], &[("dlg_id", "dlg_id")]);

	builder
});
