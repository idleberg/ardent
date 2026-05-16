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
		"${MementoUnselectedSection}",
		"${Select}",
		"${Switch}",
		"${Unless}",
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
		"${Case}",
		"${Case2}",
		"${Case3}",
		"${Case4}",
		"${Case5}",
		"${CaseElse}",
		"${Default}",
	])
});

/// Keywords that close an indentation block (e.g. `SectionEnd`, `FunctionEnd`, `!endif`).
pub static CLOSE: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!endif",
		"!macroend",
		"${EndIf}",
		"${EndSelect}",
		"${EndSwitch}",
		"${EndWhile}",
		"${Loop}",
		"${LoopUntil}",
		"${LoopWhile}",
		"${MementoSectionEnd}",
		"${Next}",
		"${While}",
		"FunctionEnd",
		"PageExEnd",
		"SectionEnd",
		"SectionGroupEnd",
	])
});

/// Keywords that sit at the opener's indent level within a block (e.g. `!else`, `${ElseIf}`).
pub static MID: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!else",
		"!elseif",
		"${Else}",
		"${ElseIf}",
		"${ElseIfNot}",
		"${ElseUnless}",
		"${AndIf}",
		"${AndIfNot}",
		"${AndUnless}",
		"${OrIf}",
		"${OrIfNot}",
		"${OrUnless}",
	])
});

/// Keywords that close the current block after being printed at the current indent level.
pub static CLOSE_AFTER: LazyLock<HashSet<String>> = LazyLock::new(|| lower_set(&["${Break}"]));
