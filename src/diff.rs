use std::io::{self, IsTerminal};

use similar::TextDiff;

/// Number of unchanged lines printed before and after each hunk, matching the
/// GNU `diff -u` default.
const CONTEXT: usize = 3;

/// Builds a unified diff between two strings.
///
/// Pass `None` as label to omit the `--- a/…` / `+++ b/…` header, e.g. when the
/// input came from stdin. Returns an empty vector when both sides are identical.
pub fn unified_diff(label: Option<&str>, original: &str, formatted: &str) -> Vec<String> {
	let text_diff = TextDiff::from_lines(original, formatted);
	let mut builder = text_diff.unified_diff();
	builder.context_radius(CONTEXT);

	if let Some(label) = label {
		builder.header(&format!("a/{label}"), &format!("b/{label}"));
	}

	let rendered = builder.to_string();

	if rendered.is_empty() {
		return Vec::new();
	}

	rendered
		.strip_suffix('\n')
		.unwrap_or(&rendered)
		.split('\n')
		.map(str::to_string)
		.collect()
}

/// Colorizes a rendered diff line. Header lines are excluded by the caller.
fn colorize(line: &str) -> String {
	if line.starts_with("@@") {
		format!("\x1b[36m{line}\x1b[0m")
	} else if line.starts_with('+') {
		format!("\x1b[32m{line}\x1b[0m")
	} else if line.starts_with('-') {
		format!("\x1b[31m{line}\x1b[0m")
	} else {
		line.to_string()
	}
}

/// Prints a unified diff for a single file to stdout. Pass `None` as label to
/// omit the `--- a/…` / `+++ b/…` header, e.g. when the input came from stdin.
pub fn print_diff(label: Option<&str>, original: &str, formatted: &str) {
	let lines = unified_diff(label, original, formatted);

	if lines.is_empty() {
		return;
	}

	if !io::stdout().is_terminal() {
		println!("{}", lines.join("\n"));
		return;
	}

	let header_lines = if label.is_some() { 2 } else { 0 };
	let painted: Vec<String> = lines
		.iter()
		.enumerate()
		.map(|(index, line)| {
			if index < header_lines {
				format!("\x1b[1m{line}\x1b[0m")
			} else {
				colorize(line)
			}
		})
		.collect();

	println!("{}", painted.join("\n"));
}

#[cfg(test)]
mod tests {
	use super::*;

	fn numbered(count: usize) -> String {
		(0..count).map(|i| format!("line {i}\n")).collect()
	}

	#[test]
	fn returns_nothing_for_identical_input() {
		assert!(unified_diff(Some("a.nsi"), "one\ntwo\n", "one\ntwo\n").is_empty());
	}

	#[test]
	fn emits_git_style_headers_without_timestamps() {
		let lines = unified_diff(Some("a.nsi"), "one\n", "two\n");

		assert_eq!(lines[0], "--- a/a.nsi");
		assert_eq!(lines[1], "+++ b/a.nsi");
	}

	#[test]
	fn omits_the_headers_when_the_label_is_none() {
		let lines = unified_diff(None, "one\n", "two\n");

		assert_eq!(lines[0], "@@ -1 +1 @@");
	}

	#[test]
	fn renders_a_replacement_hunk() {
		assert_eq!(
			unified_diff(None, "one\ntwo\nthree\n", "one\nTWO\nthree\n"),
			vec!["@@ -1,3 +1,3 @@", " one", "-two", "+TWO", " three"]
		);
	}

	#[test]
	fn keeps_three_lines_of_context_around_a_hunk() {
		let original = numbered(20);
		let formatted = original.replace("line 10\n", "LINE 10\n");

		assert_eq!(
			unified_diff(None, &original, &formatted),
			vec![
				"@@ -8,7 +8,7 @@",
				" line 7",
				" line 8",
				" line 9",
				"-line 10",
				"+LINE 10",
				" line 11",
				" line 12",
				" line 13",
			]
		);
	}

	#[test]
	fn splits_distant_changes_into_separate_hunks() {
		let original = numbered(30);
		let formatted = original
			.replace("line 2\n", "LINE 2\n")
			.replace("line 25\n", "LINE 25\n");

		let hunks: Vec<String> = unified_diff(None, &original, &formatted)
			.into_iter()
			.filter(|line| line.starts_with("@@"))
			.collect();

		assert_eq!(hunks, vec!["@@ -1,6 +1,6 @@", "@@ -23,7 +23,7 @@"]);
	}

	#[test]
	fn merges_changes_within_twice_the_context() {
		let original = numbered(20);
		let formatted = original
			.replace("line 5\n", "LINE 5\n")
			.replace("line 10\n", "LINE 10\n");

		let hunks: Vec<String> = unified_diff(None, &original, &formatted)
			.into_iter()
			.filter(|line| line.starts_with("@@"))
			.collect();

		assert_eq!(hunks, vec!["@@ -3,12 +3,12 @@"]);
	}

	#[test]
	fn reports_a_pure_insertion_with_a_zero_length_source_range() {
		assert_eq!(
			unified_diff(None, "", "one\n"),
			vec!["@@ -0,0 +1 @@", "+one"]
		);
	}

	#[test]
	fn reports_a_missing_trailing_newline() {
		assert_eq!(
			unified_diff(None, "one", "one\n"),
			vec![
				"@@ -1 +1 @@",
				"-one",
				"\\ No newline at end of file",
				"+one",
			]
		);
	}
}
