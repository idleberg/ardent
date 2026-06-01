use std::collections::HashSet;
use std::sync::LazyLock;

/// The style of a comment in an NSIS script.
#[derive(Debug, Clone, PartialEq)]
pub enum CommentStyle {
	/// A `#`-prefixed line comment.
	Hash,
	/// A `;`-prefixed line comment.
	Semicolon,
	/// A `/* ... */` block comment.
	Block,
}

/// A trailing comment attached to an instruction or label.
#[derive(Debug, Clone, PartialEq)]
pub struct TrailingComment {
	/// The comment style (`#` or `;`).
	pub style: CommentStyle,
	/// The comment text (without the leading marker).
	pub value: String,
}

/// A node in the concrete syntax tree produced by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum CSTNode {
	/// An empty line.
	Blank,
	/// A standalone comment line.
	Comment {
		/// The comment style.
		style: CommentStyle,
		/// The comment text.
		value: String,
	},
	/// An NSIS instruction (e.g. `Section`, `DetailPrint`, `!define`).
	Instruction {
		/// The instruction keyword as it appeared in source.
		keyword: String,
		/// The arguments following the keyword.
		args: Vec<String>,
		/// An optional trailing comment on the same line.
		comment: Option<TrailingComment>,
	},
	/// A label definition (e.g. `myLabel:`).
	Label {
		/// The label name (without the trailing colon).
		name: String,
		/// An optional trailing comment on the same line.
		comment: Option<TrailingComment>,
	},
}

static COMPILER_KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
	HashSet::from([
		"!addincludedir",
		"!addplugindir",
		"!appendfile",
		"!appendmemfile",
		"!assert",
		"!cd",
		"!define",
		"!delfile",
		"!echo",
		"!else",
		"!elseif",
		"!endif",
		"!error",
		"!execute",
		"!finalize",
		"!getdllversion",
		"!gettlbversion",
		"!if",
		"!ifdef",
		"!ifmacrodef",
		"!ifmacrondef",
		"!ifndef",
		"!include",
		"!insertmacro",
		"!macro",
		"!macroend",
		"!macroundef",
		"!makensis",
		"!packhdr",
		"!pragma",
		"!searchparse",
		"!searchreplace",
		"!system",
		"!tempfile",
		"!undef",
		"!uninstfinalize",
		"!verbose",
		"!warning",
	])
});

static INSTRUCTION_LOOKUP: LazyLock<HashSet<String>> = LazyLock::new(|| {
	[
		"Abort",
		"AddBrandingImage",
		"AddSize",
		"AllowRootDirInstall",
		"AllowSkipFiles",
		"AutoCloseWindow",
		"BGFont",
		"BGGradient",
		"BrandingText",
		"BringToFront",
		"Call",
		"CallInstDLL",
		"Caption",
		"ChangeUI",
		"CheckBitmap",
		"ClearErrors",
		"CompletedText",
		"ComponentText",
		"CopyFiles",
		"CPU",
		"CRCCheck",
		"CreateDirectory",
		"CreateFont",
		"CreateShortcut",
		"Delete",
		"DeleteINISec",
		"DeleteINIStr",
		"DeleteRegKey",
		"DeleteRegValue",
		"DetailPrint",
		"DetailsButtonText",
		"DirShow",
		"DirText",
		"DirVar",
		"DirVerify",
		"EnableWindow",
		"EnumRegKey",
		"EnumRegValue",
		"Exch",
		"Exec",
		"ExecShell",
		"ExecShellWait",
		"ExecWait",
		"ExpandEnvStrings",
		"File",
		"FileBufSize",
		"FileClose",
		"FileErrorText",
		"FileOpen",
		"FileRead",
		"FileReadByte",
		"FileReadUTF16LE",
		"FileReadWord",
		"FileSeek",
		"FileWrite",
		"FileWriteByte",
		"FileWriteUTF16LE",
		"FileWriteWord",
		"FindClose",
		"FindFirst",
		"FindNext",
		"FindWindow",
		"FlushINI",
		"Function",
		"FunctionEnd",
		"GetCurInstType",
		"GetCurrentAddress",
		"GetDlgItem",
		"GetDLLVersion",
		"GetDLLVersionLocal",
		"GetErrorLevel",
		"GetFileTime",
		"GetFileTimeLocal",
		"GetFullPathName",
		"GetFunctionAddress",
		"GetInstDirError",
		"GetKnownFolderPath",
		"GetLabelAddress",
		"GetRegView",
		"GetShellVarContext",
		"GetTempFileName",
		"GetWinVer",
		"Goto",
		"HideWindow",
		"Icon",
		"IfAbort",
		"IfAltRegView",
		"IfErrors",
		"IfFileExists",
		"IfRebootFlag",
		"IfRtlLanguage",
		"IfShellVarContextAll",
		"IfSilent",
		"InitPluginsDir",
		"InstallButtonText",
		"InstallColors",
		"InstallDir",
		"InstallDirRegKey",
		"InstProgressFlags",
		"InstType",
		"InstTypeGetText",
		"InstTypeSetText",
		"Int64Cmp",
		"Int64CmpU",
		"Int64Fmt",
		"IntCmp",
		"IntCmpU",
		"IntFmt",
		"IntOp",
		"IntPtrCmp",
		"IntPtrCmpU",
		"IntPtrOp",
		"IsWindow",
		"LangString",
		"LangStringUP",
		"LicenseBkColor",
		"LicenseData",
		"LicenseForceSelection",
		"LicenseLangString",
		"LicenseText",
		"LoadAndSetImage",
		"LoadLanguageFile",
		"LockWindow",
		"LogSet",
		"LogText",
		"ManifestAppendCustomString",
		"ManifestDisableWindowFiltering",
		"ManifestDPIAware",
		"ManifestDPIAwareness",
		"ManifestGdiScaling",
		"ManifestLongPathAware",
		"ManifestMaxVersionTested",
		"ManifestSupportedOS",
		"MessageBox",
		"MiscButtonText",
		"Name",
		"Nop",
		"OutFile",
		"Page",
		"PageCallbacks",
		"PageEx",
		"PageExEnd",
		"PEAddResource",
		"PEDllCharacteristics",
		"PERemoveResource",
		"PESubsysVer",
		"Pop",
		"Push",
		"Quit",
		"ReadEnvStr",
		"ReadINIStr",
		"ReadMemory",
		"ReadRegDWORD",
		"ReadRegStr",
		"Reboot",
		"RegDLL",
		"Rename",
		"RequestExecutionLevel",
		"ReserveFile",
		"Return",
		"RMDir",
		"SearchPath",
		"Section",
		"SectionEnd",
		"SectionGetFlags",
		"SectionGetInstTypes",
		"SectionGetSize",
		"SectionGetText",
		"SectionGroup",
		"SectionGroupEnd",
		"SectionIn",
		"SectionInstType",
		"SectionSetFlags",
		"SectionSetInstTypes",
		"SectionSetSize",
		"SectionSetText",
		"SendMessage",
		"SetAutoClose",
		"SetBrandingImage",
		"SetCompress",
		"SetCompressionLevel",
		"SetCompressor",
		"SetCompressorDictSize",
		"SetCtlColors",
		"SetCurInstType",
		"SetDatablockOptimize",
		"SetDateSave",
		"SetDetailsPrint",
		"SetDetailsView",
		"SetErrorLevel",
		"SetErrors",
		"SetFileAttributes",
		"SetFont",
		"SetOutPath",
		"SetOverwrite",
		"SetPluginUnload",
		"SetRebootFlag",
		"SetRegView",
		"SetShellVarContext",
		"SetSilent",
		"ShowInstDetails",
		"ShowUninstDetails",
		"ShowWindow",
		"SilentInstall",
		"SilentUnInstall",
		"Sleep",
		"SpaceTexts",
		"StrCmp",
		"StrCmpS",
		"StrCpy",
		"StrLen",
		"SubCaption",
		"SubSection",
		"SubSectionEnd",
		"Target",
		"Unicode",
		"UninstallButtonText",
		"UninstallCaption",
		"UninstallExeName",
		"UninstallIcon",
		"UninstallSubCaption",
		"UninstallText",
		"UninstPage",
		"UnRegDLL",
		"UnsafeStrCpy",
		"Var",
		"VIAddVersionKey",
		"VIFileVersion",
		"VIProductVersion",
		"WindowIcon",
		"WriteINIStr",
		"WriteRegBin",
		"WriteRegDWORD",
		"WriteRegExpandStr",
		"WriteRegMultiStr",
		"WriteRegNone",
		"WriteRegStr",
		"WriteUninstaller",
		"XPStyle",
	]
	.iter()
	.map(|kw| kw.to_lowercase())
	.collect()
});

fn is_compiler_keyword(kw: &str) -> bool {
	COMPILER_KEYWORDS.contains(kw.to_lowercase().as_str())
}

fn is_instruction_keyword(kw: &str) -> bool {
	INSTRUCTION_LOOKUP.contains(&kw.to_lowercase())
}

peg::parser! {
	grammar nsis_parser() for str {
		pub rule script() -> Vec<CSTNode>
			= items:line_items()* { items.into_iter().flatten().collect() }

		rule line_items() -> Vec<CSTNode>
			= n:block_comment() { vec![n] }
			/ n:blank_line() { vec![n] }
			/ n:comment_line() { vec![n] }
			/ label_with_instruction()
			/ n:label_line() { vec![n] }
			/ n:instruction_line() { vec![n] }

		rule blank_line() -> CSTNode
			= _() eol() { CSTNode::Blank }

		rule comment_line() -> CSTNode
			= _() s:$("#" / ";") value:$([^ '\r' | '\n']*) line_end() {
				CSTNode::Comment {
					style: if s == "#" { CommentStyle::Hash } else { CommentStyle::Semicolon },
					value: value.trim_start().to_string(),
				}
			}

		rule block_comment() -> CSTNode
			= _() "/*" value:$((!("*/") [_])*) "*/" _() line_end()? {
				CSTNode::Comment {
					style: CommentStyle::Block,
					value: value.to_string(),
				}
			}

		rule label_line() -> CSTNode
			= _() name:$(label_segment()+) ":" !":"
			  trailing:trailing_comment()? _() line_end() {
				CSTNode::Label {
					name: name.to_string(),
					comment: trailing,
				}
			}

		rule label_with_instruction() -> Vec<CSTNode>
			= _() name:$(label_segment()+) ":" !":"
			  _() kw:keyword() args:arguments() trailing:trailing_comment()? _() line_end() {
				vec![
					CSTNode::Label {
						name: name.to_string(),
						comment: None,
					},
					CSTNode::Instruction {
						keyword: kw,
						args,
						comment: trailing,
					},
				]
			}

		rule label_segment()
			= "${" label_brace_inner()* "}"
			/ ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '-' | '/']

		rule label_brace_inner()
			= "${" label_brace_inner()* "}"
			/ (!"}" [_])

		rule instruction_line() -> CSTNode
			= _() kw:keyword() args:arguments() trailing:trailing_comment()? _() line_end() {
				CSTNode::Instruction {
					keyword: kw,
					args,
					comment: trailing,
				}
			}

		rule keyword() -> String
			= compiler_keyword()
			/ macro_keyword()
			/ plugin_call_keyword()
			/ instruction_keyword()

		rule compiler_keyword() -> String
			= kw:$("!" ['a'..='z' | 'A'..='Z']+) {?
				if is_compiler_keyword(kw) { Ok(kw.to_string()) }
				else { Err("not a compiler keyword") }
			}

		rule macro_keyword() -> String
			= kw:$("${" ['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.']* "}") {
				kw.to_string()
			}

		rule plugin_call_keyword() -> String
			= kw:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']* "::" ['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {
				kw.to_string()
			}

		rule instruction_keyword() -> String
			= kw:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*) {?
				if is_instruction_keyword(kw) { Ok(kw.to_string()) }
				else { Err("not an instruction keyword") }
			}

		rule arguments() -> Vec<String>
			= args:(_() a:argument() { a })* { args }

		rule argument() -> String
			= quoted_string()
			/ bare_token()

		rule quoted_string() -> String
			= s:$(
				"\"" ("\"\"" / "$\\\"" / [^ '"' | '\r' | '\n'])* "\""
			  ) { s.to_string() }
			/ s:$(
				"'" ("$\\'" / [^ '\'' | '\r' | '\n'])* "'"
			  ) { s.to_string() }
			/ s:$(
				"`" ("$\\`" / [^ '`' | '\r' | '\n'])* "`"
			  ) { s.to_string() }

		rule bare_token() -> String
			= s:$([^ ' ' | '\t' | '\r' | '\n' | ';' | '#']+) { s.to_string() }

		rule trailing_comment() -> TrailingComment
			= _() s:$("#" / ";") value:$([^ '\r' | '\n']*) {
				TrailingComment {
					style: if s == "#" { CommentStyle::Hash } else { CommentStyle::Semicolon },
					value: value.trim_start().to_string(),
				}
			}

		rule _()
			= quiet!{[' ' | '\t']*}

		rule eol()
			= "\r\n" / "\n" / "\r"

		rule line_end()
			= eol() / eof()

		rule eof()
			= ![_]
	}
}

/// Strips a leading BOM and joins backslash-continued lines.
pub fn preprocess(input: &str) -> String {
	let without_bom = input.strip_prefix('\u{FEFF}').unwrap_or(input);
	let mut result = String::with_capacity(without_bom.len());
	let bytes = without_bom.as_bytes();
	let len = bytes.len();
	let mut i = 0;

	while i < len {
		if bytes[i] == b'\\' {
			// Check for \<newline> continuation
			if i + 1 < len && bytes[i + 1] == b'\n' {
				result.push(' ');
				i += 2;
				while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
					i += 1;
				}
				continue;
			}
			if i + 2 < len && bytes[i + 1] == b'\r' && bytes[i + 2] == b'\n' {
				result.push(' ');
				i += 3;
				while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
					i += 1;
				}
				continue;
			}
		}
		result.push(bytes[i] as char);
		i += 1;
	}

	result
}

/// Parses an NSIS script into a list of CST nodes.
pub fn parse(input: &str) -> Result<Vec<CSTNode>, String> {
	let preprocessed = preprocess(input);
	nsis_parser::script(&preprocessed).map_err(|e| format!("Parse error: {e}"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_blank_line() {
		let nodes = parse("\n").unwrap();
		assert_eq!(nodes, vec![CSTNode::Blank]);
	}

	#[test]
	fn parse_hash_comment() {
		let nodes = parse("# This is a comment\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Comment {
				style: CommentStyle::Hash,
				value: "This is a comment".to_string(),
			}]
		);
	}

	#[test]
	fn parse_semicolon_comment() {
		let nodes = parse("; Another comment\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Comment {
				style: CommentStyle::Semicolon,
				value: "Another comment".to_string(),
			}]
		);
	}

	#[test]
	fn parse_block_comment() {
		let nodes = parse("/* block */\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Comment {
				style: CommentStyle::Block,
				value: " block ".to_string(),
			}]
		);
	}

	#[test]
	fn parse_label() {
		let nodes = parse("myLabel:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "myLabel".to_string(),
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_instruction() {
		let nodes = parse("Section \"Test\"\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "Section".to_string(),
				args: vec!["\"Test\"".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_compiler_command() {
		let nodes = parse("!define FOO bar\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "!define".to_string(),
				args: vec!["FOO".to_string(), "bar".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_macro_keyword() {
		let nodes = parse("${If} $0 == 1\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "${If}".to_string(),
				args: vec!["$0".to_string(), "==".to_string(), "1".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_plugin_call() {
		let nodes = parse("nsDialogs::Create 1018\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "nsDialogs::Create".to_string(),
				args: vec!["1018".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_trailing_comment() {
		let nodes = parse("Section \"Test\" ; my comment\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "Section".to_string(),
				args: vec!["\"Test\"".to_string()],
				comment: Some(TrailingComment {
					style: CommentStyle::Semicolon,
					value: "my comment".to_string(),
				}),
			}]
		);
	}

	#[test]
	fn parse_bom_stripped() {
		let nodes = parse("\u{FEFF}Section \"Test\"\n").unwrap();
		assert_eq!(nodes.len(), 1);
		assert!(matches!(&nodes[0], CSTNode::Instruction { keyword, .. } if keyword == "Section"));
	}

	#[test]
	fn parse_line_continuation() {
		let nodes = parse("Section \\\n  \"Test\"\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "Section".to_string(),
				args: vec!["\"Test\"".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_line_continuation_crlf() {
		let nodes = parse("Section \\\r\n  \"Test\"\r\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "Section".to_string(),
				args: vec!["\"Test\"".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_empty_input() {
		let nodes = parse("").unwrap();
		assert_eq!(nodes, vec![]);
	}

	#[test]
	fn parse_multiple_instructions() {
		let nodes = parse("DetailPrint \"a\"\nDetailPrint \"b\"\n").unwrap();
		assert_eq!(nodes.len(), 2);
		assert!(
			matches!(&nodes[0], CSTNode::Instruction { keyword, .. } if keyword == "DetailPrint")
		);
		assert!(
			matches!(&nodes[1], CSTNode::Instruction { keyword, .. } if keyword == "DetailPrint")
		);
	}

	#[test]
	fn parse_label_with_trailing_comment() {
		let nodes = parse("myLabel: ; note\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "myLabel".to_string(),
				comment: Some(TrailingComment {
					style: CommentStyle::Semicolon,
					value: "note".to_string(),
				}),
			}]
		);
	}

	#[test]
	fn parse_label_with_instruction() {
		let nodes = parse("done: Return\n").unwrap();
		assert_eq!(nodes.len(), 2);
		assert_eq!(
			nodes[0],
			CSTNode::Label {
				name: "done".to_string(),
				comment: None,
			}
		);
		assert_eq!(
			nodes[1],
			CSTNode::Instruction {
				keyword: "Return".to_string(),
				args: vec![],
				comment: None,
			}
		);
	}

	#[test]
	fn parse_multiline_block_comment() {
		let nodes = parse("/* line1\nline2\nline3 */\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Comment {
				style: CommentStyle::Block,
				value: " line1\nline2\nline3 ".to_string(),
			}]
		);
	}

	#[test]
	fn parse_instruction_no_args() {
		let nodes = parse("Return\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "Return".to_string(),
				args: vec![],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_instruction_many_args() {
		let nodes = parse("WriteRegStr HKLM \"Software\\Test\" \"Key\" \"Value\"\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "WriteRegStr".to_string(),
				args: vec![
					"HKLM".to_string(),
					"\"Software\\Test\"".to_string(),
					"\"Key\"".to_string(),
					"\"Value\"".to_string(),
				],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_hash_trailing_comment() {
		let nodes = parse("DetailPrint \"hi\" # note\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "DetailPrint".to_string(),
				args: vec!["\"hi\"".to_string()],
				comment: Some(TrailingComment {
					style: CommentStyle::Hash,
					value: "note".to_string(),
				}),
			}]
		);
	}

	#[test]
	fn parse_single_quoted_arg() {
		let nodes = parse("DetailPrint 'hello world'\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "DetailPrint".to_string(),
				args: vec!["'hello world'".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_backtick_quoted_arg() {
		let nodes = parse("DetailPrint `hello world`\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "DetailPrint".to_string(),
				args: vec!["`hello world`".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_single_quoted_with_escaped_quote() {
		let nodes = parse("OutFile 'Quote $\\'This$\\''\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "OutFile".to_string(),
				args: vec!["'Quote $\\'This$\\''".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_backtick_with_escaped_backtick() {
		let nodes = parse("OutFile `Quote $\\`This$\\``\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "OutFile".to_string(),
				args: vec!["`Quote $\\`This$\\``".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_multiple_blanks() {
		let nodes = parse("\n\n\n").unwrap();
		assert_eq!(nodes, vec![CSTNode::Blank, CSTNode::Blank, CSTNode::Blank]);
	}

	#[test]
	fn parse_crlf_line_endings() {
		let nodes = parse("DetailPrint \"hi\"\r\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "DetailPrint".to_string(),
				args: vec!["\"hi\"".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn preprocess_strips_bom() {
		let result = preprocess("\u{FEFF}hello");
		assert_eq!(result, "hello");
	}

	#[test]
	fn preprocess_no_bom() {
		let result = preprocess("hello");
		assert_eq!(result, "hello");
	}

	#[test]
	fn preprocess_joins_continuation_lf() {
		let result = preprocess("foo \\\n  bar");
		assert_eq!(result, "foo  bar");
	}

	#[test]
	fn preprocess_joins_continuation_crlf() {
		let result = preprocess("foo \\\r\n  bar");
		assert_eq!(result, "foo  bar");
	}

	#[test]
	fn preprocess_no_continuation() {
		let result = preprocess("foo \\ bar");
		assert_eq!(result, "foo \\ bar");
	}

	#[test]
	fn parse_eof_without_trailing_newline() {
		let nodes = parse("DetailPrint \"hi\"").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Instruction {
				keyword: "DetailPrint".to_string(),
				args: vec!["\"hi\"".to_string()],
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_dotted_label() {
		let nodes = parse(".onInit:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: ".onInit".to_string(),
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_label_with_variable_interpolation() {
		let nodes = parse("WordFunc_WordFind${_WORDFUNC_S}_restart:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "WordFunc_WordFind${_WORDFUNC_S}_restart".to_string(),
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_label_with_nested_variable_interpolation() {
		let nodes = parse("${${_Logic}Else}:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "${${_Logic}Else}".to_string(),
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_hyphenated_label() {
		let nodes = parse("LineFind-TextCompare:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "LineFind-TextCompare".to_string(),
				comment: None,
			}]
		);
	}

	#[test]
	fn parse_label_with_slash() {
		let nodes = parse("WordFunc_WordAdd${_S}_/word:\n").unwrap();
		assert_eq!(
			nodes,
			vec![CSTNode::Label {
				name: "WordFunc_WordAdd${_S}_/word".to_string(),
				comment: None,
			}]
		);
	}
}
