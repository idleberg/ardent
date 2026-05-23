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

Pre-commit hooks are managed by lefthook (auto-formats and lints on commit).

## NSIS Language Reference

When adding or modifying support for NSIS commands/instructions, always verify syntax and parameters against the authoritative source:

```
makensis -CMDHELP <command>   # show help for a specific command
makensis -CMDHELP             # list all available commands
```

This is the ground truth for command names, parameter order, and valid options.

## Testing Requirements

- Every new feature and bugfix must include a corresponding test.
- When modifying formatter behavior, verify that existing tests still pass — and update them if the expected output intentionally changed.
- Run `mise run test` (or the full `mise run checks`) to confirm.

## Code Style

- Rust edition 2024
- Indentation: tabs (see `.editorconfig`)
- `cargo fmt` handles Rust formatting automatically (enforced by hooks)
- `#![warn(missing_docs)]` is enabled — public items need doc comments
