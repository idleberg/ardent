use std::fs;
use std::process::Command;

const UNFORMATTED: &str = "Section \"demo\"\n  DetailPrint \"x\"\nSectionEnd\n";
const FORMATTED: &str = "Section \"demo\"\n\tDetailPrint \"x\"\nSectionEnd\n";
// `StrStr` is not an NSIS instruction, so this cannot be parsed.
const UNPARSEABLE: &str = "Section \"demo\"\n\tStrStr $0 \"a\" \"b\"\nSectionEnd\n";

struct Run {
	stdout: String,
	code: Option<i32>,
}

fn run(subcommand: &str, args: &[&str]) -> Run {
	let output = Command::new(env!("CARGO_BIN_EXE_ardent"))
		.arg(subcommand)
		.args(args)
		.output()
		.expect("ardent runs");

	Run {
		stdout: String::from_utf8(output.stdout).expect("stdout is valid UTF-8"),
		code: output.status.code(),
	}
}

#[test]
fn format_exits_with_an_error_when_a_file_cannot_be_parsed() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("broken.nsi");
	fs::write(&file, UNPARSEABLE).unwrap();

	let result = run("format", &[file.to_str().unwrap()]);

	// Nothing is written to stdout for an unparseable file, so exiting 0 here would look
	// exactly like formatting a file to nothing.
	assert_eq!(result.code, Some(2));
	assert!(result.stdout.is_empty(), "{}", result.stdout);
}

#[test]
fn format_exits_with_an_error_even_when_other_files_succeed() {
	let dir = tempfile::tempdir().unwrap();
	let good = dir.path().join("good.nsi");
	let bad = dir.path().join("broken.nsi");
	fs::write(&good, FORMATTED).unwrap();
	fs::write(&bad, UNPARSEABLE).unwrap();

	let result = run("format", &[good.to_str().unwrap(), bad.to_str().unwrap()]);

	assert_eq!(result.code, Some(2));
	assert_eq!(result.stdout, FORMATTED);
}

#[test]
fn format_exits_successfully_for_a_parseable_file() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("dirty.nsi");
	fs::write(&file, UNFORMATTED).unwrap();

	let result = run("format", &[file.to_str().unwrap()]);

	assert_eq!(result.code, Some(0));
	assert_eq!(result.stdout, FORMATTED);
}

#[test]
fn check_exits_with_an_error_when_a_file_cannot_be_parsed() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("broken.nsi");
	fs::write(&file, UNPARSEABLE).unwrap();

	let result = run("check", &[file.to_str().unwrap()]);

	// 2 (error) rather than 1 (formatting issues) — the file was never checked at all.
	assert_eq!(result.code, Some(2));
}

#[test]
fn check_reports_an_error_over_formatting_issues() {
	let dir = tempfile::tempdir().unwrap();
	let dirty = dir.path().join("dirty.nsi");
	let bad = dir.path().join("broken.nsi");
	fs::write(&dirty, UNFORMATTED).unwrap();
	fs::write(&bad, UNPARSEABLE).unwrap();

	let result = run("check", &[dirty.to_str().unwrap(), bad.to_str().unwrap()]);

	assert_eq!(result.code, Some(2));
}
