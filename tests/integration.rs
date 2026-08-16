use ardent::{EndOfLine, Formatter, FormatterOptions};

fn formatter_lf() -> Formatter {
	Formatter::new(FormatterOptions {
		end_of_line: Some(EndOfLine::Lf),
		..Default::default()
	})
	.unwrap()
}

#[test]
fn format_fixture_example1() {
	let input = include_str!("./fixtures/example1.nsi");
	let f = formatter_lf();
	let result = f.format(input).unwrap();
	assert!(result.contains("Section \"\""));
	assert!(result.contains("\tSetOutPath $INSTDIR"));
	assert!(result.contains("SectionEnd\n"));
}

#[test]
fn format_fixture_example2() {
	let input = include_str!("./fixtures/example2.nsi");
	let f = formatter_lf();
	let result = f.format(input).unwrap();
	assert!(result.contains("Section \"Example2 (required)\""));
	assert!(result.contains("\tWriteRegStr HKLM"));
	assert!(result.contains("\tWriteUninstaller"));
}

#[test]
fn format_fixture_bigtest() {
	let input = include_str!("./fixtures/bigtest.nsi");
	let f = formatter_lf();
	assert!(f.format(input).is_ok());
}

#[test]
fn idempotent_example1() {
	let input = include_str!("./fixtures/example1.nsi");
	let f = formatter_lf();
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn idempotent_example2() {
	let input = include_str!("./fixtures/example2.nsi");
	let f = formatter_lf();
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn switch_case_fallthrough() {
	let f = formatter_lf();
	let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Case} 2\nDetailPrint \"two\"\n${EndSwitch}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${Case} 2\n\t\tDetailPrint \"two\"\n${EndSwitch}\n"
	);
}

#[test]
fn switch_case_with_break() {
	let f = formatter_lf();
	let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Break}\n${Case} 2\nDetailPrint \"two\"\n${Break}\n${EndSwitch}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\t\t${Break}\n\n\t${Case} 2\n\t\tDetailPrint \"two\"\n\t\t${Break}\n${EndSwitch}\n"
	);
}

#[test]
fn switch_case_idempotent() {
	let f = formatter_lf();
	let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Case} 2\nDetailPrint \"two\"\n${Break}\n${CaseElse}\nDetailPrint \"else\"\n${EndSwitch}\n";
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn switch_case_else_alias() {
	let f = formatter_lf();
	let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Case_Else}\nDetailPrint \"else\"\n${EndSwitch}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${Case_Else}\n\t\tDetailPrint \"else\"\n${EndSwitch}\n"
	);
}

#[test]
fn memento_section_ex_indent() {
	let f = formatter_lf();
	let input = "${MementoSectionEx} \"\" \"x\" mid sid\nNop\n${MementoSectionEnd}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${MementoSectionEx} \"\" \"x\" mid sid\n\tNop\n${MementoSectionEnd}\n"
	);
}

#[test]
fn canonical_include_logiclib_loops() {
	let f = formatter_lf();
	let input = "${while} $0 < 3\n${endwhile}\n${for} $0 1 3\n${next}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${While} $0 < 3\n${EndWhile}\n\n${For} $0 1 3\n${Next}\n"
	);
}

#[test]
fn canonical_include_logiclib_unless() {
	let f = formatter_lf();
	let input = "${unless} $R0 == \"\"\n${endunless}\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${Unless} $R0 == \"\"\n${EndUnless}\n");
}

#[test]
fn while_end_while_indent() {
	let f = formatter_lf();
	let input = "Section \"x\"\n${While} $0 < 3\nNop\n${EndWhile}\nNop\nSectionEnd\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"Section \"x\"\n\t${While} $0 < 3\n\t\tNop\n\t${EndWhile}\n\n\tNop\nSectionEnd\n"
	);
}

#[test]
fn unless_end_unless_indent() {
	let f = formatter_lf();
	let input = "Section \"x\"\n${Unless} $0 == 1\nNop\n${EndUnless}\nNop\nSectionEnd\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"Section \"x\"\n\t${Unless} $0 == 1\n\t\tNop\n\t${EndUnless}\n\n\tNop\nSectionEnd\n"
	);
}

#[test]
fn canonical_include_logiclib() {
	let f = formatter_lf();
	let input = "${if} $R0 == \"\"\n${endif}\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${If} $R0 == \"\"\n${EndIf}\n");
}

#[test]
fn canonical_include_filefunc() {
	let f = formatter_lf();
	let input = "${getsize} \"$INSTDIR\" \"/S=0K\" $0 $1 $2\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${GetSize} \"$INSTDIR\" \"/S=0K\" $0 $1 $2\n");
}

#[test]
fn canonical_include_winver() {
	let f = formatter_lf();
	let input = "${atleastwin8.1} $0\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${AtLeastWin8.1} $0\n");
}

#[test]
fn canonical_include_x64() {
	let f = formatter_lf();
	let input = "${runningx64} $0\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${RunningX64} $0\n");
}

#[test]
fn canonical_include_strfunc() {
	let f = formatter_lf();
	let input = "${strrep} $0 \"hello world\" \"world\" \"there\"\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${StrRep} $0 \"hello world\" \"world\" \"there\"\n");
}

#[test]
fn canonical_include_wordfunc() {
	let f = formatter_lf();
	let input = "${versioncompare} $0 $1 $2\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${VersionCompare} $0 $1 $2\n");
}

#[test]
fn canonical_include_textfunc() {
	let f = formatter_lf();
	let input = "${configread} \"$INSTDIR\\config.ini\" \"Key=\" $0\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${ConfigRead} \"$INSTDIR\\config.ini\" \"Key=\" $0\n"
	);
}

#[test]
fn canonical_include_memento() {
	let f = formatter_lf();
	let input = "${mementosection} \"MySection\" SEC_MY\n${mementosectionend}\n";
	let result = f.format(input).unwrap();
	assert_eq!(
		result,
		"${MementoSection} \"MySection\" SEC_MY\n${MementoSectionEnd}\n"
	);
}

#[test]
fn canonical_include_unknown_macro_unchanged() {
	let f = formatter_lf();
	let input = "${MyCustomMacro} \"arg\"\n";
	let result = f.format(input).unwrap();
	assert_eq!(result, "${MyCustomMacro} \"arg\"\n");
}

#[test]
fn canonical_include_idempotent() {
	let f = formatter_lf();
	let input =
		"${GETSIZE} \"$INSTDIR\" \"/S=0K\" $0 $1 $2\n${RUNNINGX64} $0\n${IF} $0 == 1\n${ENDIF}\n";
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn intop_unsigned_right_shift() {
	let f = formatter_lf();
	assert_eq!(
		f.format("IntOp $0 $1>>>$2\n").unwrap(),
		"IntOp $0 $1 >>> $2\n"
	);
}

#[test]
fn intop_unsigned_right_shift_already_spaced() {
	let f = formatter_lf();
	assert_eq!(
		f.format("IntOp $0 $1 >>> $2\n").unwrap(),
		"IntOp $0 $1 >>> $2\n"
	);
}

#[test]
fn intop_right_shift_not_confused_with_unsigned() {
	let f = formatter_lf();
	assert_eq!(
		f.format("IntOp $0 $1>>$2\n").unwrap(),
		"IntOp $0 $1 >> $2\n"
	);
}

#[test]
fn format_fixture_quotes() {
	let input = include_str!("./fixtures/quotes.nsi");
	let f = formatter_lf();
	let result = f.format(input).unwrap();
	assert!(result.contains("DetailPrint \"installer\""));
	assert!(result.contains("DetailPrint 'Installer with \"quote\"'"));
	assert!(result.contains("DetailPrint \"Installer with 'quote'\""));
	assert!(result.contains("DetailPrint `She said \"it's done\"`"));
	assert!(result.contains("DetailPrint \"All $\\\"three$\\\" 'quote' `types`\""));
}

#[test]
fn idempotent_quotes() {
	let input = include_str!("./fixtures/quotes.nsi");
	let f = formatter_lf();
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn format_fixture_unicode() {
	let input = include_str!("./fixtures/unicode.nsi");
	let f = formatter_lf();
	let result = f.format(input).unwrap();
	assert!(result.contains("DetailPrint \"שלום, עולם!\""));
	assert!(result.contains("DetailPrint \"مرحبا بالعالم!\""));
	assert!(result.contains("DetailPrint \"こんにちは、世界！\""));
	assert!(result.contains("DetailPrint \"你好，世界！\""));
	assert!(result.contains("DetailPrint \"привет, мир!\""));
	assert!(result.contains("DetailPrint \"안녕하세요!\""));
	assert!(result.contains("DetailPrint \"สวัสดีชาวโลก!\""));
	assert!(result.contains("DetailPrint \"Γεια σου, Κόσμε!\""));
}

#[test]
fn idempotent_unicode() {
	let input = include_str!("./fixtures/unicode.nsi");
	let f = formatter_lf();
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn format_unicode_with_bom() {
	let input = "\u{FEFF}; BOM test\nDetailPrint \"שלום\"\n";
	let f = formatter_lf();
	let result = f.format(input).unwrap();
	assert_eq!(result, "; BOM test\nDetailPrint \"שלום\"\n");
}

#[test]
fn format_fixture_variables() {
	let input = include_str!("./fixtures/variables.nsi");
	let f = formatter_lf();
	let result = f.format(input).unwrap();

	// Built-in variables, defines and language strings are canonicalized
	assert!(result.contains("\tSetOutPath $INSTDIR"));
	assert!(result.contains("\tStrCpy $R0 \"$INSTDIR\\bin\""));
	assert!(result.contains("\tStrCpy $CustomVar \"$INSTDIR$TEMP\""));
	assert!(result.contains("!addincludedir \"${NSISDIR}\\Include\""));
	assert!(result.contains("InstallDir \"$PROGRAMFILES\\Ardent\""));
	assert!(result.contains("\tDetailPrint \"$(^Completed) $(^Name)\""));

	// Custom names, environment variables and escapes are left as typed
	assert!(result.contains("\tDetailPrint \"${MyOwnDefine}\""));
	assert!(result.contains("\tDetailPrint \"$myOwnVariable\""));
	assert!(result.contains("\tDetailPrint \"$(MyOwnLangString)\""));
	assert!(result.contains("\tDetailPrint \"$%windir%\\system32\""));
	assert!(result.contains("\tDetailPrint \"$$instdir is escaped\""));
}

#[test]
fn idempotent_variables() {
	let input = include_str!("./fixtures/variables.nsi");
	let f = formatter_lf();
	let first = f.format(input).unwrap();
	let second = f.format(&first).unwrap();
	assert_eq!(first, second);
}

#[test]
fn error_on_zero_indent_size_with_spaces() {
	let result = Formatter::new(FormatterOptions {
		use_tabs: false,
		indent_size: 0,
		..Default::default()
	});
	assert!(result.is_err());
}
