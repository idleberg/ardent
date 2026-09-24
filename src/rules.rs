//! Block-structure rules: which keywords open, close or continue a block.
//!
//! Generated from the Dent Style Specification 0.1.0 by `mise run spec:codegen`.
//! Do not edit — change the spec instead.

use std::collections::HashSet;
use std::sync::LazyLock;

fn lower_set(keywords: &[&str]) -> HashSet<String> {
	keywords.iter().map(|k| k.to_lowercase()).collect()
}

/// Keywords that open a new indentation block (e.g. `Section`, `Function`, `!if`).
pub static OPEN: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!if",
		"!ifdef",
		"!ifmacrodef",
		"!ifmacrondef",
		"!ifndef",
		"!macro",
		"${Do}",
		"${DoUntil}",
		"${DoWhile}",
		"${For}",
		"${ForEach}",
		"${If}",
		"${IfNot}",
		"${MementoSection}",
		"${MementoSectionEx}",
		"${MementoUnselectedSection}",
		"${Select}",
		"${Switch}",
		"${Unless}",
		"${While}",
		"Function",
		"PageEx",
		"Section",
		"SectionGroup",
	])
});

/// Keywords that open a case arm within a switch/select block.
/// These print one level inside their parent and indent their body one further level,
/// without pushing to the indent stack.
pub static CASE: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"${Case_Else}",
		"${Case}",
		"${Case2}",
		"${Case3}",
		"${Case4}",
		"${Case5}",
		"${CaseElse}",
		"${Default}",
	])
});

/// Keywords that close an indentation block (e.g. `SectionEnd`, `!endif`).
pub static CLOSE: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!endif",
		"!macroend",
		"${EndIf}",
		"${EndSelect}",
		"${EndSwitch}",
		"${EndUnless}",
		"${EndWhile}",
		"${Loop}",
		"${LoopUntil}",
		"${LoopWhile}",
		"${MementoSectionEnd}",
		"${Next}",
		"FunctionEnd",
		"PageExEnd",
		"SectionEnd",
		"SectionGroupEnd",
	])
});

/// Keywords printed at the opening keyword’s level without changing the current depth (e.g. `${Else}`).
pub static MID: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!else",
		"${AndIf}",
		"${AndIfNot}",
		"${AndUnless}",
		"${Else}",
		"${ElseIf}",
		"${ElseIfNot}",
		"${ElseUnless}",
		"${OrIf}",
		"${OrIfNot}",
		"${OrUnless}",
	])
});

/// Keywords printed at the current level that then close the block (e.g. `${Break}`).
pub static CLOSE_AFTER: LazyLock<HashSet<String>> = LazyLock::new(|| lower_set(&["${Break}"]));
