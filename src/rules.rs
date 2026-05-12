use std::collections::HashSet;
use std::sync::LazyLock;

fn lower_set(keywords: &[&str]) -> HashSet<String> {
	keywords.iter().map(|k| k.to_lowercase()).collect()
}

pub static OPEN: LazyLock<HashSet<String>> = LazyLock::new(|| {
	lower_set(&[
		"!if",
		"!ifdef",
		"!ifmacrodef",
		"!ifmacrondef",
		"!ifndef",
		"!macro",
		"${Case}",
		"${Case2}",
		"${Case3}",
		"${Case4}",
		"${Case5}",
		"${CaseElse}",
		"${Default}",
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

pub static CLOSE_AFTER: LazyLock<HashSet<String>> = LazyLock::new(|| lower_set(&["${Break}"]));
