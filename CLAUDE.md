# Ardent

Opinionated formatter for [NSIS](https://nsis.sourceforge.io/) scripts. Parses NSIS source into a concrete syntax tree (PEG-based), then pretty-prints with canonical casing, consistent indentation, and normalized parameters.

## Project Structure

- `src/lib.rs` — public API (`Formatter`, `FormatterOptions`)
- `src/main.rs` — CLI (`ardent format`, `ardent check`) built with clap
- `src/parser.rs` — PEG grammar (via `peg` crate) producing a CST
- `src/printer.rs` — CST → formatted NSIS source
- `src/rules.rs` — block-structure rules (which keywords open/close/continue blocks)
- `src/canonical_casing.rs` — instruction name → canonical case lookup
- `src/canonical_includes.rs` — bundled include library macro casing
- `src/canonical_parameters.rs` — parameter casing lookup
- `tests/` — integration tests with fixture files in `tests/fixtures/`
- `tasks/compare.ts` — Bun script comparing output against the Node.js predecessor (`@nsis/dent`)

## Tooling

Everything runs through [mise](https://mise.jdx.dev/). Key tasks:

```
mise run checks          # format:check + lint + test
mise run format          # cargo fmt
mise run format:check    # cargo fmt --check
mise run lint            # cargo clippy -- --deny warnings
mise run test            # cargo test
mise run build           # cargo build --release
mise run compare -- <files>  # compare against @nsis/dent output
```

Pre-commit hooks are managed by hk (auto-formats and lints on commit).

## NSIS Language Reference

When adding or modifying support for NSIS commands/instructions, always verify syntax and parameters against the authoritative source:

```
makensis -CMDHELP <command>   # show help for a specific command
makensis -CMDHELP             # list all available commands
```

This is the ground truth for command names, parameter order, and valid options.

## Testing Requirements

- Every new feature and bugfix must include a corresponding test.
- When modifying formatter behavior, verify that existing tests still pass — and update them if the expected output intentionally changed. Editing existing tests always requires user confirmation.
- Run `mise run test` (or the full `mise run checks`) to confirm.

## Code Style

- Rust edition 2024
- Indentation: tabs (see `.editorconfig`)
- `cargo fmt` handles Rust formatting automatically (enforced by hooks)
- `#![warn(missing_docs)]` is enabled — public items need doc comments

## MCP Tools: code-review-graph

**IMPORTANT: This project has a knowledge graph. ALWAYS use the
code-review-graph MCP tools BEFORE using Grep/Glob/Read to explore
the codebase.** The graph is faster, cheaper (fewer tokens), and gives
you structural context (callers, dependents, test coverage) that file
scanning cannot.

### When to use graph tools FIRST

- **Exploring code**: `semantic_search_nodes_tool` or `query_graph_tool` instead of Grep
- **Understanding impact**: `get_impact_radius_tool` instead of manually tracing imports
- **Code review**: `detect_changes_tool` + `get_review_context_tool` instead of reading entire files
- **Finding relationships**: `query_graph_tool` with callers_of/callees_of/imports_of/tests_for
- **Architecture questions**: `get_architecture_overview_tool` + `list_communities_tool`

Fall back to Grep/Glob/Read **only** when the graph doesn't cover what you need.

### Key Tools

| Tool                             | Use when                                               |
| -------------------------------- | ------------------------------------------------------ |
| `detect_changes_tool`            | Reviewing code changes — gives risk-scored analysis    |
| `get_review_context_tool`        | Need source snippets for review — token-efficient      |
| `get_impact_radius_tool`         | Understanding blast radius of a change                 |
| `get_affected_flows_tool`        | Finding which execution paths are impacted             |
| `query_graph_tool`               | Tracing callers, callees, imports, tests, dependencies |
| `semantic_search_nodes_tool`     | Finding functions/classes by name or keyword           |
| `get_architecture_overview_tool` | Understanding high-level codebase structure            |
| `refactor_tool`                  | Planning renames, finding dead code                    |

### Workflow

1. The graph auto-updates on file changes (via hooks).
2. Use `detect_changes_tool` for code review.
3. Use `get_affected_flows_tool` to understand impact.
4. Use `query_graph_tool` pattern="tests_for" to check coverage.
