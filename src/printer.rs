use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::canonical_casing::CANONICAL_CASING;
use crate::canonical_includes::CANONICAL_INCLUDES;
use crate::canonical_parameters::{
	GLOBAL_PARAMETER_PREFIXES, GLOBAL_PARAMETERS, INSTRUCTION_PARAMETERS,
};
use crate::parser::{CSTNode, CommentStyle, TrailingComment};
use crate::rules::{CASE, CLOSE, CLOSE_AFTER, MID, OPEN};

/// Options controlling the printer output.
pub struct PrinterOptions {
	/// Whether to indent with tabs.
	pub use_tabs: bool,
	/// Number of spaces per indent level (ignored when `use_tabs` is `true`).
	pub indent_size: usize,
	/// Whether to collapse consecutive blank lines and strip leading/trailing blanks.
	pub trim_empty_lines: bool,
	/// The line ending string to use.
	pub eol: String,
}

/// Renders a list of CST nodes into a formatted NSIS script string.
pub fn print(nodes: &[CSTNode], options: &PrinterOptions) -> String {
	let mut level: usize = 0;
	let mut stack: Vec<usize> = Vec::new();
	let mut lines: Vec<String> = Vec::new();

	let mut processed = ensure_blank_around_blocks(nodes);
	if options.trim_empty_lines {
		processed = trim_and_collapse_blanks(&processed);
	}

	for node in &processed {
		match node {
			CSTNode::Blank => lines.push(String::new()),
			CSTNode::Comment { style, value } => {
				lines.push(print_comment(style, value, level, options));
			}
			CSTNode::Label { name, comment } => {
				lines.push(print_label(name, comment.as_ref(), level, options));
			}
			CSTNode::Instruction {
				keyword,
				args,
				comment,
			} => {
				let kw = keyword.to_lowercase();

				if OPEN.contains(&kw) {
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						level,
						options,
					));
					stack.push(level);
					level += 1;
				} else if CASE.contains(&kw) {
					let parent_level = stack.last().copied().unwrap_or(0);
					let case_level = parent_level + 1;
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						case_level,
						options,
					));
					level = case_level + 1;
				} else if CLOSE.contains(&kw) {
					level = stack.pop().unwrap_or(0);
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						level,
						options,
					));
				} else if MID.contains(&kw) {
					let opener_level = stack.last().copied().unwrap_or(0);
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						opener_level,
						options,
					));
				} else if CLOSE_AFTER.contains(&kw) {
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						level,
						options,
					));
					level = stack.last().copied().unwrap_or(0) + 1;
				} else {
					lines.push(print_instruction(
						keyword,
						args,
						comment.as_ref(),
						level,
						options,
					));
				}
			}
		}
	}

	let mut result = lines.join(&options.eol);
	result.push_str(&options.eol);
	result
}

fn indent_str(level: usize, options: &PrinterOptions) -> String {
	if options.use_tabs {
		"\t".repeat(level)
	} else {
		" ".repeat(options.indent_size * level)
	}
}

fn print_comment(
	style: &CommentStyle,
	value: &str,
	level: usize,
	options: &PrinterOptions,
) -> String {
	let prefix = indent_str(level, options);

	if *style == CommentStyle::Block {
		let comment_lines: Vec<&str> = value.split('\n').collect();

		if comment_lines.len() == 1 {
			return format!("{prefix}/*{value}*/");
		}

		comment_lines
			.iter()
			.enumerate()
			.map(|(i, line)| {
				let line = line.strip_suffix('\r').unwrap_or(line);
				if i == 0 {
					format!("{prefix}/*{line}")
				} else {
					let stripped = line.trim_start();
					if i == comment_lines.len() - 1 {
						format!("{prefix} {stripped}*/")
					} else {
						format!("{prefix} {stripped}")
					}
				}
			})
			.collect::<Vec<_>>()
			.join(&options.eol)
	} else {
		let marker = if *style == CommentStyle::Hash {
			'#'
		} else {
			';'
		};
		format!("{prefix}{marker} {value}")
	}
}

fn print_label(
	name: &str,
	comment: Option<&TrailingComment>,
	level: usize,
	options: &PrinterOptions,
) -> String {
	let mut line = format!("{}{}:", indent_str(level, options), name);
	if let Some(c) = comment {
		line.push(' ');
		line.push_str(&print_trailing_comment(c));
	}
	line
}

fn normalize_arg(arg: &str, instr_params: Option<&HashMap<&str, &str>>) -> String {
	if arg.starts_with('"') || arg.starts_with('\'') || arg.starts_with('`') || arg.starts_with('$')
	{
		return arg.to_string();
	}

	let lower = arg.to_lowercase();

	if let Some(params) = instr_params
		&& let Some(&canonical) = params.get(lower.as_str())
	{
		return canonical.to_string();
	}
	if let Some(&canonical) = GLOBAL_PARAMETERS.get(lower.as_str()) {
		return canonical.to_string();
	}

	if let Some(eq_idx) = arg.find('=')
		&& eq_idx > 0
	{
		let prefix_lower = &lower[..=eq_idx];
		if let Some(&canonical) = GLOBAL_PARAMETER_PREFIXES.get(prefix_lower) {
			return format!("{}{}", canonical, &arg[eq_idx + 1..]);
		}
	}

	arg.to_string()
}

fn split_pipe_tokens(args: &[String]) -> Vec<String> {
	args.iter()
		.flat_map(|arg| {
			if arg.starts_with('"') || arg.starts_with('\'') || arg.starts_with('`') {
				return vec![arg.clone()];
			}
			if !arg.contains('|') || arg == "|" {
				return vec![arg.clone()];
			}
			split_preserving_groups(arg, '|')
		})
		.collect()
}

fn split_preserving_groups(arg: &str, sep: char) -> Vec<String> {
	let mut result = Vec::new();
	let mut current = String::new();
	let chars: Vec<char> = arg.chars().collect();
	let mut i = 0;

	while i < chars.len() {
		if chars[i] == '$'
			&& i + 1 < chars.len()
			&& chars[i + 1] == '{'
			&& let Some(end) = arg[i + 2..].find('}')
		{
			let group = &arg[i..i + 2 + end + 1];
			current.push_str(group);
			i += 2 + end + 1;
			continue;
		}

		if chars[i] == sep {
			if !current.is_empty() {
				result.push(current.clone());
				current.clear();
			}
			result.push(sep.to_string());
			i += 1;
			continue;
		}

		current.push(chars[i]);
		i += 1;
	}

	if !current.is_empty() {
		result.push(current);
	}
	result
}

static ARITHMETIC_INSTRUCTIONS: LazyLock<HashSet<&'static str>> =
	LazyLock::new(|| HashSet::from(["intop", "intptrop"]));

static ARITHMETIC_OPS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
	HashSet::from([
		"||", "&&", "<<", ">>", "+", "-", "*", "/", "%", "|", "&", "^", "~", "!",
	])
});

static SINGLE_CHAR_OPS: LazyLock<HashSet<char>> =
	LazyLock::new(|| HashSet::from(['+', '-', '*', '/', '%', '|', '&', '^', '~', '!']));

fn split_arithmetic_tokens(args: &[String]) -> Vec<String> {
	args.iter()
		.flat_map(|arg| {
			if arg.starts_with('"') || arg.starts_with('\'') || arg.starts_with('`') {
				return vec![arg.clone()];
			}
			if ARITHMETIC_OPS.contains(arg.as_str()) {
				return vec![arg.clone()];
			}
			tokenize_arithmetic(arg)
		})
		.collect()
}

fn tokenize_arithmetic(arg: &str) -> Vec<String> {
	let mut result = Vec::new();
	let mut current = String::new();
	let mut last_was_op = true;
	let chars: Vec<char> = arg.chars().collect();
	let mut i = 0;

	while i < chars.len() {
		if chars[i] == '$'
			&& i + 1 < chars.len()
			&& chars[i + 1] == '{'
			&& let Some(end) = arg[i + 2..].find('}')
		{
			let group = &arg[i..i + 2 + end + 1];
			current.push_str(group);
			i += 2 + end + 1;
			last_was_op = false;
			continue;
		}

		if i + 1 < chars.len() {
			let two: String = chars[i..=i + 1].iter().collect();
			if ARITHMETIC_OPS.contains(two.as_str()) {
				if !current.is_empty() {
					result.push(current.clone());
					current.clear();
				}
				result.push(two);
				last_was_op = true;
				i += 2;
				continue;
			}
		}

		if SINGLE_CHAR_OPS.contains(&chars[i]) {
			if chars[i] == '-' && last_was_op {
				current.push(chars[i]);
				i += 1;
				continue;
			}
			if !current.is_empty() {
				result.push(current.clone());
				current.clear();
			}
			result.push(chars[i].to_string());
			last_was_op = true;
			i += 1;
			continue;
		}

		current.push(chars[i]);
		last_was_op = false;
		i += 1;
	}

	if !current.is_empty() {
		result.push(current);
	}

	if result.is_empty() {
		vec![arg.to_string()]
	} else {
		result
	}
}

fn join_with_compact_pipes(args: &[String]) -> String {
	let mut result = String::new();
	for (i, arg) in args.iter().enumerate() {
		if arg == "|" {
			result.push('|');
		} else if i > 0 && args[i - 1] == "|" {
			result.push_str(arg);
		} else {
			if !result.is_empty() {
				result.push(' ');
			}
			result.push_str(arg);
		}
	}
	result
}

fn print_instruction(
	keyword: &str,
	args: &[String],
	comment: Option<&TrailingComment>,
	level: usize,
	options: &PrinterOptions,
) -> String {
	let kw_lower = keyword.to_lowercase();
	let canonical_kw = CANONICAL_CASING
		.get(kw_lower.as_str())
		.or_else(|| CANONICAL_INCLUDES.get(kw_lower.as_str()))
		.copied()
		.unwrap_or(keyword);
	let instr_params = INSTRUCTION_PARAMETERS.get(kw_lower.as_str());
	let is_arithmetic = ARITHMETIC_INSTRUCTIONS.contains(kw_lower.as_str());

	let split_args = if is_arithmetic {
		split_arithmetic_tokens(args)
	} else {
		split_pipe_tokens(args)
	};

	let normalized: Vec<String> = split_args
		.iter()
		.map(|a| normalize_arg(a, instr_params))
		.collect();

	let joined = if is_arithmetic {
		normalized.join(" ")
	} else {
		join_with_compact_pipes(&normalized)
	};

	let parts = if normalized.is_empty() {
		canonical_kw.to_string()
	} else {
		format!("{canonical_kw} {joined}")
	};

	let mut line = format!("{}{}", indent_str(level, options), parts);

	if let Some(c) = comment {
		line.push(' ');
		line.push_str(&print_trailing_comment(c));
	}

	line
}

fn print_trailing_comment(comment: &TrailingComment) -> String {
	let marker = if comment.style == CommentStyle::Hash {
		'#'
	} else {
		';'
	};
	format!("{marker} {}", comment.value)
}

fn is_block_open(node: &CSTNode) -> bool {
	matches!(node, CSTNode::Instruction { keyword, .. } if {
		let kw = keyword.to_lowercase();
		OPEN.contains(&kw) || CASE.contains(&kw)
	})
}

fn is_block_close(node: &CSTNode) -> bool {
	matches!(node, CSTNode::Instruction { keyword, .. } if CLOSE.contains(&keyword.to_lowercase()))
}

fn ensure_blank_around_blocks(nodes: &[CSTNode]) -> Vec<CSTNode> {
	let mut result: Vec<CSTNode> = Vec::new();
	let mut prev_non_blank: Option<&CSTNode> = None;

	for (i, node) in nodes.iter().enumerate() {
		let last_is_blank = result.last().is_some_and(|n| matches!(n, CSTNode::Blank));

		if let Some(prev) = prev_non_blank
			&& !last_is_blank
			&& !matches!(node, CSTNode::Blank)
		{
			if is_block_open(node)
				&& !is_block_open(prev)
				&& !matches!(prev, CSTNode::Comment { .. })
			{
				result.push(CSTNode::Blank);
			} else if matches!(node, CSTNode::Comment { .. })
				&& !is_block_open(prev)
				&& !matches!(prev, CSTNode::Comment { .. })
			{
				let mut j = i + 1;
				while j < nodes.len()
					&& matches!(nodes[j], CSTNode::Blank | CSTNode::Comment { .. })
				{
					j += 1;
				}
				if j < nodes.len() && is_block_open(&nodes[j]) {
					result.push(CSTNode::Blank);
				}
			} else if is_block_close(prev) && !is_block_close(node) && !is_block_open(node) {
				result.push(CSTNode::Blank);
			}
		}

		result.push(node.clone());
		if !matches!(node, CSTNode::Blank) {
			prev_non_blank = Some(node);
		}
	}

	result
}

fn trim_and_collapse_blanks(nodes: &[CSTNode]) -> Vec<CSTNode> {
	let mut start = 0;
	while start < nodes.len() && matches!(nodes[start], CSTNode::Blank) {
		start += 1;
	}

	let mut end = nodes.len();
	while end > start && matches!(nodes[end - 1], CSTNode::Blank) {
		end -= 1;
	}

	let mut result = Vec::new();
	let mut prev_blank = false;

	for node in &nodes[start..end] {
		if matches!(node, CSTNode::Blank) {
			if !prev_blank {
				result.push(node.clone());
				prev_blank = true;
			}
		} else {
			result.push(node.clone());
			prev_blank = false;
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::parser::parse;

	fn format_with_defaults(input: &str) -> String {
		let nodes = parse(input).unwrap();
		print(
			&nodes,
			&PrinterOptions {
				use_tabs: true,
				indent_size: 2,
				trim_empty_lines: true,
				eol: "\n".to_string(),
			},
		)
	}

	#[test]
	fn format_section_block() {
		let result = format_with_defaults("section \"Test\"\nDetailPrint \"hello\"\nsectionend\n");
		assert_eq!(
			result,
			"Section \"Test\"\n\tDetailPrint \"hello\"\nSectionEnd\n"
		);
	}

	#[test]
	fn format_nested_blocks() {
		let input = "section \"Test\"\n!if 1\nDetailPrint \"hi\"\n!endif\nsectionend\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"Section \"Test\"\n\t!if 1\n\t\tDetailPrint \"hi\"\n\t!endif\nSectionEnd\n"
		);
	}

	#[test]
	fn format_canonical_casing() {
		let result = format_with_defaults("detailprint \"hello\"\n");
		assert_eq!(result, "DetailPrint \"hello\"\n");
	}

	#[test]
	fn format_pipe_compact() {
		let result = format_with_defaults("MessageBox MB_OK|MB_DEFBUTTON1 \"Hi\"\n");
		assert_eq!(result, "MessageBox MB_OK|MB_DEFBUTTON1 \"Hi\"\n");
	}

	#[test]
	fn format_arithmetic_spaced() {
		let result = format_with_defaults("IntOp $0 $1+$2\n");
		assert_eq!(result, "IntOp $0 $1 + $2\n");
	}

	#[test]
	fn format_mid_keyword_at_opener_level() {
		let result =
			format_with_defaults("!if 1\nDetailPrint \"a\"\n!else\nDetailPrint \"b\"\n!endif\n");
		assert_eq!(
			result,
			"!if 1\n\tDetailPrint \"a\"\n!else\n\tDetailPrint \"b\"\n!endif\n"
		);
	}

	#[test]
	fn format_close_after_keyword() {
		let result = format_with_defaults(
			"${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Break}\n${EndSwitch}\n",
		);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\t\t${Break}\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_unknown_keyword_no_casing_change() {
		let result = format_with_defaults("myPlugin::DoStuff arg1\n");
		assert_eq!(result, "myPlugin::DoStuff arg1\n");
	}

	#[test]
	fn format_label_indented_in_block() {
		let result =
			format_with_defaults("Section \"Test\"\nmyLabel:\nDetailPrint \"hi\"\nSectionEnd\n");
		assert_eq!(
			result,
			"Section \"Test\"\n\tmyLabel:\n\tDetailPrint \"hi\"\nSectionEnd\n"
		);
	}

	#[test]
	fn format_label_with_comment() {
		let result = format_with_defaults("myLabel: ; note\n");
		assert_eq!(result, "myLabel: ; note\n");
	}

	#[test]
	fn format_hash_comment() {
		let result = format_with_defaults("# my comment\n");
		assert_eq!(result, "# my comment\n");
	}

	#[test]
	fn format_semicolon_comment() {
		let result = format_with_defaults("; my comment\n");
		assert_eq!(result, "; my comment\n");
	}

	#[test]
	fn format_block_comment_single_line() {
		let result = format_with_defaults("/* hello */\n");
		assert_eq!(result, "/* hello */\n");
	}

	#[test]
	fn format_block_comment_multiline() {
		let result = format_with_defaults("/* line1\n  line2\n  line3 */\n");
		assert!(result.starts_with("/*"));
		assert!(result.contains("line2"));
		assert!(result.contains("*/\n"));
	}

	#[test]
	fn format_comment_indented_in_block() {
		let result =
			format_with_defaults("Section \"Test\"\n; comment\nDetailPrint \"hi\"\nSectionEnd\n");
		assert!(result.contains("\t; comment\n"));
	}

	#[test]
	fn format_normalize_global_param() {
		let result = format_with_defaults("CopyFiles /silent \"a\" \"b\"\n");
		assert_eq!(result, "CopyFiles /SILENT \"a\" \"b\"\n");
	}

	#[test]
	fn format_normalize_instruction_param() {
		let result = format_with_defaults("WriteRegStr hklm \"Key\" \"Name\" \"Val\"\n");
		assert_eq!(result, "WriteRegStr HKLM \"Key\" \"Name\" \"Val\"\n");
	}

	#[test]
	fn format_normalize_param_prefix() {
		let result = format_with_defaults("LangString msg /lang=1033 \"Hello\"\n");
		assert!(result.contains("/LANG=1033"));
	}

	#[test]
	fn format_quoted_args_not_normalized() {
		let result = format_with_defaults("DetailPrint \"hklm\"\n");
		assert_eq!(result, "DetailPrint \"hklm\"\n");
	}

	#[test]
	fn format_dollar_args_not_normalized() {
		let result = format_with_defaults("StrCpy $0 $INSTDIR\n");
		assert_eq!(result, "StrCpy $0 $INSTDIR\n");
	}

	#[test]
	fn format_pipe_preserves_macro_vars() {
		let result = format_with_defaults("MessageBox ${MB_TYPE}|MB_ICONQUESTION \"Sure?\"\n");
		assert!(result.contains("${MB_TYPE}|MB_ICONQUESTION"));
	}

	#[test]
	fn format_arithmetic_two_char_op() {
		let result = format_with_defaults("IntOp $0 $1<<2\n");
		assert_eq!(result, "IntOp $0 $1 << 2\n");
	}

	#[test]
	fn format_arithmetic_negative_operand() {
		let result = format_with_defaults("IntOp $0 0-$1\n");
		assert_eq!(result, "IntOp $0 0 - $1\n");
	}

	#[test]
	fn format_spaces_indent() {
		let nodes = parse("section \"Test\"\nDetailPrint \"hi\"\nsectionend\n").unwrap();
		let result = print(
			&nodes,
			&PrinterOptions {
				use_tabs: false,
				indent_size: 4,
				trim_empty_lines: true,
				eol: "\n".to_string(),
			},
		);
		assert_eq!(
			result,
			"Section \"Test\"\n    DetailPrint \"hi\"\nSectionEnd\n"
		);
	}

	#[test]
	fn format_trim_consecutive_blanks() {
		let result = format_with_defaults("DetailPrint \"a\"\n\n\n\nDetailPrint \"b\"\n");
		assert_eq!(result, "DetailPrint \"a\"\n\nDetailPrint \"b\"\n");
	}

	#[test]
	fn format_trim_leading_blanks() {
		let result = format_with_defaults("\n\n\nDetailPrint \"a\"\n");
		assert_eq!(result, "DetailPrint \"a\"\n");
	}

	#[test]
	fn format_trim_trailing_blanks() {
		let result = format_with_defaults("DetailPrint \"a\"\n\n\n");
		assert_eq!(result, "DetailPrint \"a\"\n");
	}

	#[test]
	fn format_blank_before_block() {
		let result = format_with_defaults(
			"DetailPrint \"before\"\nSection \"Test\"\nDetailPrint \"in\"\nSectionEnd\n",
		);
		assert!(result.contains("DetailPrint \"before\"\n\nSection \"Test\""));
	}

	#[test]
	fn format_blank_after_block() {
		let result = format_with_defaults(
			"Section \"Test\"\nDetailPrint \"in\"\nSectionEnd\nDetailPrint \"after\"\n",
		);
		assert!(result.contains("SectionEnd\n\nDetailPrint \"after\""));
	}

	#[test]
	fn format_no_extra_blank_between_blocks() {
		let result = format_with_defaults(
			"Section \"A\"\nDetailPrint \"a\"\nSectionEnd\nSection \"B\"\nDetailPrint \"b\"\nSectionEnd\n",
		);
		assert!(result.contains("SectionEnd\n\nSection \"B\""));
		assert!(!result.contains("\n\n\n"));
	}

	#[test]
	fn format_instruction_no_args() {
		let result = format_with_defaults("Return\n");
		assert_eq!(result, "Return\n");
	}

	#[test]
	fn format_deeply_nested() {
		let input =
			"Section \"A\"\n!if 1\n!ifdef FOO\nDetailPrint \"deep\"\n!endif\n!endif\nSectionEnd\n";
		let result = format_with_defaults(input);
		assert!(result.contains("\t\t\tDetailPrint \"deep\""));
	}

	#[test]
	fn normalize_arg_preserves_unknown_bare() {
		let result = normalize_arg("UNKNOWN_TOKEN", None);
		assert_eq!(result, "UNKNOWN_TOKEN");
	}

	#[test]
	fn split_pipe_tokens_basic() {
		let args = vec!["MB_OK|MB_ICONQUESTION".to_string()];
		let result = split_pipe_tokens(&args);
		assert_eq!(result, vec!["MB_OK", "|", "MB_ICONQUESTION"]);
	}

	#[test]
	fn split_pipe_tokens_quoted_not_split() {
		let args = vec!["\"foo|bar\"".to_string()];
		let result = split_pipe_tokens(&args);
		assert_eq!(result, vec!["\"foo|bar\""]);
	}

	#[test]
	fn join_with_compact_pipes_basic() {
		let args = vec!["MB_OK".to_string(), "|".to_string(), "MB_ICON".to_string()];
		let result = join_with_compact_pipes(&args);
		assert_eq!(result, "MB_OK|MB_ICON");
	}

	#[test]
	fn tokenize_arithmetic_simple() {
		let result = tokenize_arithmetic("$1+$2");
		assert_eq!(result, vec!["$1", "+", "$2"]);
	}

	#[test]
	fn tokenize_arithmetic_with_macro_var() {
		let result = tokenize_arithmetic("${Var}+1");
		assert_eq!(result, vec!["${Var}", "+", "1"]);
	}

	#[test]
	fn format_case_fallthrough() {
		let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Case} 2\nDetailPrint \"two\"\n${EndSwitch}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${Case} 2\n\t\tDetailPrint \"two\"\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_case_with_break() {
		let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Break}\n${Case} 2\nDetailPrint \"two\"\n${Break}\n${EndSwitch}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\t\t${Break}\n\n\t${Case} 2\n\t\tDetailPrint \"two\"\n\t\t${Break}\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_case_else() {
		let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${CaseElse}\nDetailPrint \"else\"\n${EndSwitch}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${CaseElse}\n\t\tDetailPrint \"else\"\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_default_case() {
		let input = "${Switch} $0\n${Case} 1\nDetailPrint \"one\"\n${Default}\nDetailPrint \"def\"\n${EndSwitch}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${Default}\n\t\tDetailPrint \"def\"\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_nested_switch() {
		let input = "${Switch} $0\n${Case} 1\n${Switch} $1\n${Case} a\nDetailPrint \"nested\"\n${EndSwitch}\n${EndSwitch}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Switch} $0\n\t${Case} 1\n\t\t${Switch} $1\n\t\t\t${Case} a\n\t\t\t\tDetailPrint \"nested\"\n\t\t${EndSwitch}\n${EndSwitch}\n"
		);
	}

	#[test]
	fn format_select_with_cases() {
		let input = "${Select} $0\n${Case} 1\nDetailPrint \"one\"\n${Case} 2\nDetailPrint \"two\"\n${EndSelect}\n";
		let result = format_with_defaults(input);
		assert_eq!(
			result,
			"${Select} $0\n\t${Case} 1\n\t\tDetailPrint \"one\"\n\n\t${Case} 2\n\t\tDetailPrint \"two\"\n${EndSelect}\n"
		);
	}

	#[test]
	fn format_include_macro_casing() {
		let input = "${if} $R0 == \"\"\n${endif}\n";
		let result = format_with_defaults(input);
		assert_eq!(result, "${If} $R0 == \"\"\n${EndIf}\n");
	}

	#[test]
	fn format_filefunc_macro_casing() {
		let input = "${GETSIZE} \"$INSTDIR\" \"/S=0K\" $0 $1 $2\n";
		let result = format_with_defaults(input);
		assert_eq!(result, "${GetSize} \"$INSTDIR\" \"/S=0K\" $0 $1 $2\n");
	}

	#[test]
	fn format_winver_macro_with_dot() {
		let input = "${atleastwin8.1} $0\n";
		let result = format_with_defaults(input);
		assert_eq!(result, "${AtLeastWin8.1} $0\n");
	}

	#[test]
	fn format_x64_macro_casing() {
		let input = "${runningx64} $0\n";
		let result = format_with_defaults(input);
		assert_eq!(result, "${RunningX64} $0\n");
	}

	#[test]
	fn format_unknown_macro_unchanged() {
		let input = "${MyCustomMacro} \"arg\"\n";
		let result = format_with_defaults(input);
		assert_eq!(result, "${MyCustomMacro} \"arg\"\n");
	}
}
