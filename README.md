# ardent

[![License](https://img.shields.io/github/license/idleberg/ardent?color=blue&style=for-the-badge)](https://github.com/idleberg/ardent/blob/main/LICENSE)
[![Version](https://img.shields.io/crates/v/ardent?style=for-the-badge)](https://crates.io/crates/ardent)
[![CI](https://img.shields.io/github/actions/workflow/status/idleberg/ardent/ci.yml?style=for-the-badge)](https://github.com/idleberg/ardent/actions)

> An opinionated code formatter for NSIS scripts

## Description

This is a Rust implementation of [`dent-cli`](https://www.npmjs.org/package/@nsis/dent-cli), a NodeJS-based formatting tool for NSIS scripts.

> **"ar"** for **R**ust, **"dent"** as for the formatter. Nevermind!

It aims to be fully compatible while making distribution easier for people outside the NodeJS ecosystem. 

## Installation

### crates.io

```sh
cargo install ardent
```

### Source

```sh
git clone https://github.com/idleberg/ardent.git
cd ardent
cargo build --release
```

The binary is at `target/release/ardent`.

## CLI Usage

```
ardent [OPTIONS] [COMMAND]

Commands:
  format  Format NSIS scripts
  check   Check if NSIS scripts are formatted correctly

Options:
  -D, --debug    Print debug messages
  -h, --help     Print help
  -V, --version  Print version
```

### Format

Formats one or more `.nsi` / `.nsh` files.

```sh
# Print formatted output to stdout
ardent format installer.nsi

# Edit files in-place
ardent format --write src/**/*.nsi
```

See `ardent format --help` for available options.

### Check

Checks whether files are already formatted.

```sh
# Check only (reports drift)
ardent check src/**/*.nsi

# Check and auto-fix (still exits 1 if drift was found)
ardent check --write src/**/*.nsi
```

See `ardent check --help` for available options.

## Library Usage

### Formatting

```rust
use ardent::{DentOptions, EndOfLines, Formatter};

let formatter = Formatter::new(DentOptions {
    end_of_lines: Some(EndOfLines::Lf),
    use_tabs: true,
    indent_size: 2,
    trim_empty_lines: true,
}).expect("valid options");

let input = r#"section "My Section"
detailprint "Hello"
sectionend
"#;

let output = formatter.format(input).expect("valid NSIS");
assert_eq!(output, "Section \"My Section\"\n\tDetailPrint \"Hello\"\nSectionEnd\n");
```

### Checking

Returns `None` if the input is already formatted, or `Some(formatted)` if it needs changes.

```rust
use ardent::{DentOptions, Formatter};

let formatter = Formatter::new(DentOptions::default()).unwrap();

match formatter.check(input).unwrap() {
    None => println!("Already formatted"),
    Some(formatted) => println!("Needs formatting"),
}
```

### Options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `end_of_lines` | `Option<EndOfLines>` | `None` (auto-detect) | Force CRLF or LF line endings |
| `indent_size` | `usize` | `2` | Spaces per indent level (ignored when using tabs) |
| `trim_empty_lines` | `bool` | `true` | Collapse consecutive blank lines and strip leading/trailing blanks |
| `use_tabs` | `bool` | `true` | Indent with tabs instead of spaces |

:white_check_mark: [Why defaulting to tabs is good for accessibility](https://github.com/prettier/prettier/issues/7475#issuecomment-668544890)

## License

This work is licensed under [The MIT License](LICENSE).
