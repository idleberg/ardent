pub mod canonical_casing;
pub mod canonical_parameters;
pub mod parser;
pub mod printer;
pub mod rules;

use parser::parse;
use printer::{PrinterOptions, print};

const DEFAULT_INDENT_SIZE: usize = 2;

#[derive(Debug, Clone)]
pub enum EndOfLines {
	Crlf,
	Lf,
}

#[derive(Debug, Clone)]
pub struct DentOptions {
	pub end_of_lines: Option<EndOfLines>,
	pub indent_size: usize,
	pub trim_empty_lines: bool,
	pub use_tabs: bool,
}

impl Default for DentOptions {
	fn default() -> Self {
		Self {
			end_of_lines: None,
			indent_size: DEFAULT_INDENT_SIZE,
			trim_empty_lines: true,
			use_tabs: true,
		}
	}
}

pub struct Formatter {
	options: DentOptions,
}

impl Formatter {
	pub fn new(options: DentOptions) -> Result<Self, String> {
		if !options.use_tabs
			&& options.indent_size == 0 {
				return Err("The indent_size option expects a positive integer".to_string());
			}
		Ok(Self { options })
	}

	pub fn format(&self, input: &str) -> Result<String, String> {
		let nodes = parse(input)?;
		let eol = self.detect_eol(input);

		Ok(print(
			&nodes,
			&PrinterOptions {
				use_tabs: self.options.use_tabs,
				indent_size: self.options.indent_size,
				trim_empty_lines: self.options.trim_empty_lines,
				eol,
			},
		))
	}

	pub fn check(&self, input: &str) -> Result<Option<String>, String> {
		let formatted = self.format(input)?;
		if formatted == input {
			Ok(None)
		} else {
			Ok(Some(formatted))
		}
	}

	fn detect_eol(&self, input: &str) -> String {
		if let Some(ref eol) = self.options.end_of_lines {
			return match eol {
				EndOfLines::Crlf => "\r\n".to_string(),
				EndOfLines::Lf => "\n".to_string(),
			};
		}

		if input.contains("\r\n") || cfg!(windows) {
			"\r\n".to_string()
		} else {
			"\n".to_string()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn format_basic() {
		let formatter = Formatter::new(DentOptions {
			end_of_lines: Some(EndOfLines::Lf),
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
		let formatter = Formatter::new(DentOptions {
			end_of_lines: Some(EndOfLines::Lf),
			..Default::default()
		})
		.unwrap();

		let input = "Section \"Test\"\n\tDetailPrint \"hello\"\nSectionEnd\n";
		assert!(formatter.check(input).unwrap().is_none());
	}

	#[test]
	fn check_returns_some_when_unformatted() {
		let formatter = Formatter::new(DentOptions {
			end_of_lines: Some(EndOfLines::Lf),
			..Default::default()
		})
		.unwrap();

		let input = "section \"Test\"\nDetailPrint \"hello\"\nsectionend\n";
		assert!(formatter.check(input).unwrap().is_some());
	}

	#[test]
	fn spaces_indent() {
		let formatter = Formatter::new(DentOptions {
			end_of_lines: Some(EndOfLines::Lf),
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
