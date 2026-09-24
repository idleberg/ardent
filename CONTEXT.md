# Ardent

A formatter that renders NSIS scripts in Dent style. Ardent parses a script into a concrete syntax
tree and prints it back out; the style it prints is defined elsewhere, by the Dent Style
Specification.

## Language

### The style and its authority

**Dent style**:
The formatting style Ardent implements: casing, indentation, blank lines, quoting and wrapping. The
style is named Dent and belongs to no single tool.
_Avoid_: Ardent style, our formatting

**Dent Style Specification**:
The document, data tables and conformance cases that define Dent style, published as
`@nsis/dent-spec`. It is the authority: where Ardent and the specification disagree, Ardent is wrong,
and a rule change belongs in the specification first.
_Avoid_: The spec (ambiguous with the PEG grammar), the reference implementation

**Implementation**:
A tool that formats NSIS scripts in Dent style. Ardent is one; `@nsis/dent` is the other.
_Avoid_: Port, clone (Ardent is not downstream of Dent)

**Conformance case**:
One directory of input bytes, expected output bytes and optional options that Ardent must reproduce
exactly, run by `tests/conformance.rs`.
_Avoid_: Fixture (a fixture is only an input; a case carries its expectation)

**Table**:
A style decision the specification publishes as data — a canonical spelling, a parameter casing, a
keyword's block role. The `src/canonical_*.rs` and `src/rules.rs` modules are generated from these
and are not edited by hand.
_Avoid_: Lookup, dictionary, map

### Inside the formatter

**Concrete syntax tree**:
The parse result that keeps every byte of the input represented, blank lines and comments included,
so printing can be lossless.
_Avoid_: AST (an abstract tree discards what the formatter must preserve), parse tree

**Canonical spelling**:
The exact casing a keyword, parameter or built-in name must be printed with. Lookups are keyed by
the lowercased spelling.
_Avoid_: Correct casing, proper name

**Block role**:
What a keyword does to indentation — `open`, `close`, `mid`, `case` or `closeAfter`. Every block
keyword has exactly one role.
_Avoid_: Keyword type, category

**Idempotency**:
The property that formatting an already-formatted script returns it unchanged. Required of every
conformance case.
_Avoid_: Stability, convergence
