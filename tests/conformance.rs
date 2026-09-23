//! Runs the conformance cases of the Dent Style Specification.
//!
//! The cases are the authority: where a case and Ardent disagree, Ardent is wrong. Cases come
//! from the installed `@nsis/dent-spec`, so the suite grows when the spec is upgraded.

use ardent::{CommentStyle, EndOfLine, Formatter, FormatterOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Locates the spec package, via `DENT_SPEC_DIR` or the installed dependency.
///
/// Returns `None` when the spec is not installed, so `cargo test` still works without the
/// Node toolchain. CI installs it and runs `mise run spec:check`, which fails if it is missing.
fn spec_dir() -> Option<PathBuf> {
	let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

	let candidates = [
		std::env::var("DENT_SPEC_DIR").ok().map(PathBuf::from),
		Some(manifest.join("node_modules/@nsis/dent-spec")),
	];

	candidates
		.into_iter()
		.flatten()
		.find(|candidate| candidate.join("cases").is_dir())
}

/// Every case directory, as the `<area>/<name>` id the spec cites.
fn find_cases(root: &Path, dir: &Path, found: &mut Vec<String>) {
	let mut entries: Vec<PathBuf> = fs::read_dir(dir)
		.unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()))
		.filter_map(Result::ok)
		.map(|entry| entry.path())
		.filter(|path| path.is_dir())
		.collect();

	entries.sort();

	for entry in entries {
		if entry.join("input.nsi").is_file() {
			let id = entry
				.strip_prefix(root)
				.expect("case inside cases/")
				.to_string_lossy()
				.into_owned();
			found.push(id);
		} else {
			find_cases(root, &entry, found);
		}
	}
}

/// Reads a case's `options.toml`, which holds only `key = value` pairs and comments.
fn read_options(case_dir: &Path) -> FormatterOptions {
	let mut options = FormatterOptions::default();

	let Ok(contents) = fs::read_to_string(case_dir.join("options.toml")) else {
		return options;
	};

	for line in contents.lines() {
		let line = line.trim();

		if line.is_empty() || line.starts_with('#') {
			continue;
		}

		let (key, value) = line
			.split_once('=')
			.unwrap_or_else(|| panic!("unparsable options.toml line: {line}"));
		let key = key.trim();
		let value = value.trim().trim_matches('"');

		match key {
			"comment_style" => {
				options.comment_style = Some(match value {
					"hash" => CommentStyle::Hash,
					"semi" => CommentStyle::Semi,
					other => panic!("unknown comment_style: {other}"),
				});
			}
			"end_of_line" => {
				options.end_of_line = Some(match value {
					"lf" => EndOfLine::Lf,
					"crlf" => EndOfLine::Crlf,
					other => panic!("unknown end_of_line: {other}"),
				});
			}
			"indent_size" => options.indent_size = value.parse().expect("integer indent_size"),
			"print_width" => options.print_width = value.parse().expect("integer print_width"),
			"single_quote" => options.single_quote = value == "true",
			"trim_empty_lines" => options.trim_empty_lines = value == "true",
			"use_tabs" => options.use_tabs = value == "true",
			other => panic!("unknown option in options.toml: {other}"),
		}
	}

	options
}

#[test]
fn conformance_cases() {
	let Some(spec) = spec_dir() else {
		eprintln!(
			"skipping conformance cases: @nsis/dent-spec is not installed \
			 (run `pnpm install`, or set DENT_SPEC_DIR)"
		);
		return;
	};

	let cases_dir = spec.join("cases");

	let mut cases = Vec::new();
	find_cases(&cases_dir, &cases_dir, &mut cases);

	assert!(!cases.is_empty(), "no conformance cases found");

	let mut failures = Vec::new();

	for id in &cases {
		let case_dir = cases_dir.join(id);
		let input = fs::read_to_string(case_dir.join("input.nsi")).expect("case input");
		let formatter = Formatter::new(read_options(&case_dir));

		if case_dir.join("error").is_file() {
			// Invalid options (§3) are rejected when the formatter is built, not when it formats.
			if formatter.is_ok_and(|formatter| formatter.format(&input).is_ok()) {
				failures.push(format!("{id}: expected an error, but formatting succeeded"));
			}
			continue;
		}

		let expected = fs::read_to_string(case_dir.join("output.nsi")).expect("case output");
		let formatter = formatter.unwrap_or_else(|e| panic!("{id}: invalid case options: {e}"));

		match formatter.format(&input) {
			Err(e) => failures.push(format!("{id}: formatting failed: {e}")),
			Ok(actual) if actual != expected => failures.push(format!(
				"{id}:\n    expected: {expected:?}\n    actual:   {actual:?}"
			)),
			Ok(actual) => {
				// §1.3: formatting is idempotent.
				match formatter.format(&actual) {
					Ok(second) if second != actual => {
						failures.push(format!("{id}: not idempotent"));
					}
					Err(e) => failures.push(format!("{id}: reformatting failed: {e}")),
					Ok(_) => {}
				}
			}
		}
	}

	assert!(
		failures.is_empty(),
		"{} of {} conformance cases failed:\n{}",
		failures.len(),
		cases.len(),
		failures.join("\n")
	);
}
