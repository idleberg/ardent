use std::fs;
use std::process::Command;

const UNFORMATTED: &str = "Section \"demo\"\n  DetailPrint \"x\"\nSectionEnd\n";
const FORMATTED: &str = "Section \"demo\"\n\tDetailPrint \"x\"\nSectionEnd\n";

struct Run {
	stdout: String,
	code: Option<i32>,
}

fn check(args: &[&str]) -> Run {
	let output = Command::new(env!("CARGO_BIN_EXE_ardent"))
		.arg("check")
		.args(args)
		.output()
		.expect("ardent runs");

	Run {
		stdout: String::from_utf8(output.stdout).expect("stdout is valid UTF-8"),
		code: output.status.code(),
	}
}

#[test]
fn diff_prints_a_unified_diff_for_a_drifting_file() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("dirty.nsi");
	fs::write(&file, UNFORMATTED).unwrap();

	let run = check(&["--diff", file.to_str().unwrap()]);

	assert_eq!(run.code, Some(1));
	assert!(run.stdout.contains("--- a/"), "{}", run.stdout);
	assert!(run.stdout.contains("+++ b/"), "{}", run.stdout);
	assert!(run.stdout.contains("@@ -1,3 +1,3 @@"), "{}", run.stdout);
	assert!(
		run.stdout.contains("-  DetailPrint \"x\""),
		"{}",
		run.stdout
	);
	assert!(
		run.stdout.contains("+\tDetailPrint \"x\""),
		"{}",
		run.stdout
	);
}

#[test]
fn diff_prints_nothing_for_an_already_formatted_file() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("clean.nsi");
	fs::write(&file, FORMATTED).unwrap();

	let run = check(&["--diff", file.to_str().unwrap()]);

	assert_eq!(run.code, Some(0));
	assert!(run.stdout.is_empty(), "{}", run.stdout);
}

#[test]
fn diff_and_write_both_apply_the_fix_and_show_it() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("dirty.nsi");
	fs::write(&file, UNFORMATTED).unwrap();

	let run = check(&["--diff", "--write", file.to_str().unwrap()]);

	assert_eq!(run.code, Some(1));
	assert!(run.stdout.contains("@@ -1,3 +1,3 @@"), "{}", run.stdout);
	assert_eq!(fs::read_to_string(&file).unwrap(), FORMATTED);
}

#[test]
fn silent_suppresses_the_diff() {
	let dir = tempfile::tempdir().unwrap();
	let file = dir.path().join("dirty.nsi");
	fs::write(&file, UNFORMATTED).unwrap();

	let run = check(&["--diff", "--silent", file.to_str().unwrap()]);

	assert_eq!(run.code, Some(1));
	assert!(run.stdout.is_empty(), "{}", run.stdout);
}
