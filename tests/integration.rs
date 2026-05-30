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
fn error_on_zero_indent_size_with_spaces() {
	let result = Formatter::new(FormatterOptions {
		use_tabs: false,
		indent_size: 0,
		..Default::default()
	});
	assert!(result.is_err());
}
