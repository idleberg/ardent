//! Opinionated formatter for [NSIS](https://nsis.sourceforge.io/) scripts.
//!
//! Ardent parses NSIS scripts into a concrete syntax tree, then pretty-prints them with
//! canonical casing, consistent indentation, and normalized parameters.
//!
//! # Examples
//!
//! ```
//! use ardent::{Formatter, FormatterOptions, EndOfLine};
//!
//! let formatter = Formatter::new(FormatterOptions {
//!     end_of_line: Some(EndOfLine::Lf),
//!     ..Default::default()
//! }).unwrap();
//!
//! let input = "section \"Hello World\"\ndetailprint \"hi\"\nsectionend\n";
//! let output = formatter.format(input).unwrap();
//! assert_eq!(output, "Section \"Hello World\"\n\tDetailPrint \"hi\"\nSectionEnd\n");
//! ```
//!
//! For CLI usage, see the [repository](https://github.com/idleberg/ardent).

#![warn(missing_docs)]

/// Canonical casing lookup table for NSIS instructions.
pub mod canonical_casing;
/// Canonical casing lookup table for NSIS bundled include library macros.
pub mod canonical_includes;
/// Canonical parameter lookup tables for NSIS instructions.
pub mod canonical_parameters;
/// PEG-based parser that produces a concrete syntax tree from NSIS source.
pub mod parser;
/// Pretty-printer that renders a CST back to formatted NSIS source.
pub mod printer;
/// Block-structure rules defining which keywords open, close, or continue blocks.
pub mod rules;

use parser::parse;
use printer::print;

const DEFAULT_INDENT_SIZE: usize = 2;
const DEFAULT_PRINT_WIDTH: usize = 120;

/// Line ending style for formatted output.
#[derive(Debug, Clone)]
pub enum EndOfLine {
	/// Windows-style line endings (`\r\n`).
	Crlf,
	/// Unix-style line endings (`\n`).
	Lf,
}

/// Configuration for the formatter.
///
/// # Examples
///
/// ```
/// use ardent::FormatterOptions;
///
/// // Use defaults (tabs, indent size 2, trim empty lines)
/// let opts = FormatterOptions::default();
///
/// // Use 4-space indentation
/// let opts = FormatterOptions {
///     use_tabs: false,
///     indent_size: 4,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct FormatterOptions {
	/// Line ending style. When `None`, the formatter auto-detects from the input.
	pub end_of_line: Option<EndOfLine>,
	/// Number of spaces per indent level (ignored when `use_tabs` is `true`).
	pub indent_size: usize,
	/// Whether to collapse consecutive blank lines and strip leading/trailing blanks.
	pub trim_empty_lines: bool,
	/// Whether to indent with tabs (`true`) or spaces (`false`).
	pub use_tabs: bool,
	/// Maximum line width before breaking with `\` continuations. `0` disables wrapping.
	pub print_width: usize,
	/// Whether to prefer single quotes (`true`) or double quotes (`false`).
	pub single_quote: bool,
}

impl Default for FormatterOptions {
	fn default() -> Self {
		Self {
			end_of_line: None,
			indent_size: DEFAULT_INDENT_SIZE,
			trim_empty_lines: true,
			use_tabs: true,
			print_width: DEFAULT_PRINT_WIDTH,
			single_quote: false,
		}
	}
}

/// An NSIS script formatter.
///
/// Parses the input into a concrete syntax tree, applies canonical casing and
/// indentation rules, then prints the result.
///
/// # Examples
///
/// ```
/// use ardent::{Formatter, FormatterOptions, EndOfLine};
///
/// let formatter = Formatter::new(FormatterOptions {
///     end_of_line: Some(EndOfLine::Lf),
///     ..Default::default()
/// }).unwrap();
///
/// // Format a script
/// let formatted = formatter.format("detailprint \"hi\"\n").unwrap();
/// assert_eq!(formatted, "DetailPrint \"hi\"\n");
///
/// // Check whether a script is already formatted
/// let result = formatter.check("DetailPrint \"hi\"\n").unwrap();
/// assert!(result.is_none()); // None means already formatted
/// ```
pub struct Formatter {
	options: FormatterOptions,
}

impl Formatter {
	/// Creates a new formatter with the given options.
	///
	/// Returns an error if `use_tabs` is `false` and `indent_size` is zero.
	pub fn new(options: FormatterOptions) -> Result<Self, String> {
		if !options.use_tabs && options.indent_size == 0 {
			return Err("The indent_size option expects a positive integer".to_string());
		}
		Ok(Self { options })
	}

	/// Formats an NSIS script, returning the formatted source.
	///
	/// Returns an error if the input cannot be parsed.
	pub fn format(&self, input: &str) -> Result<String, String> {
		let nodes = parse(input)?;
		let eol = self.detect_eol(input);

		Ok(print(&nodes, &self.options, &eol))
	}

	/// Checks whether an NSIS script is already formatted.
	///
	/// Returns `Ok(None)` if the input matches the formatted output, or
	/// `Ok(Some(formatted))` with the formatted version if it differs.
	pub fn check(&self, input: &str) -> Result<Option<String>, String> {
		let formatted = self.format(input)?;
		if formatted == input {
			Ok(None)
		} else {
			Ok(Some(formatted))
		}
	}

	fn detect_eol(&self, input: &str) -> String {
		if let Some(ref eol) = self.options.end_of_line {
			return match eol {
				EndOfLine::Crlf => "\r\n".to_string(),
				EndOfLine::Lf => "\n".to_string(),
			};
		}

		if input.contains('\n') && !input.contains("\r\n") {
			"\n".to_string()
		} else {
			"\r\n".to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn format_basic() {
		let formatter = Formatter::new(FormatterOptions {
			end_of_line: Some(EndOfLine::Lf),
			..Default::default()
		})
		.unwrap();

		let result = formatter
			.format("section \"Test\"\nDetailPrint \"hello\"\nsectionend\n")
			.unwrap();
		assert_eq!(
			result,
			"Section \"Test\"\n\tDetailPrint \"hello\"\nSectionEnd\n"
		);
	}

	#[test]
	fn check_returns_none_when_formatted() {
		let formatter = Formatter::new(FormatterOptions {
			end_of_line: Some(EndOfLine::Lf),
			..Default::default()
		})
		.unwrap();

		let input = "Section \"Test\"\n\tDetailPrint \"hello\"\nSectionEnd\n";
		assert!(formatter.check(input).unwrap().is_none());
	}

	#[test]
	fn check_returns_some_when_unformatted() {
		let formatter = Formatter::new(FormatterOptions {
			end_of_line: Some(EndOfLine::Lf),
			..Default::default()
		})
		.unwrap();

		let input = "section \"Test\"\nDetailPrint \"hello\"\nsectionend\n";
		assert!(formatter.check(input).unwrap().is_some());
	}

	#[test]
	fn spaces_indent() {
		let formatter = Formatter::new(FormatterOptions {
			end_of_line: Some(EndOfLine::Lf),
			use_tabs: false,
			indent_size: 4,
			..Default::default()
		})
		.unwrap();

		let result = formatter
			.format("section \"Test\"\nDetailPrint \"hello\"\nsectionend\n")
			.unwrap();
		assert_eq!(
			result,
			"Section \"Test\"\n    DetailPrint \"hello\"\nSectionEnd\n"
		);
	}
}
