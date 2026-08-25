---
title: Command-line usage
description: Learn how to use the Ardent CLI tool.
---

The Rust crate for Ardent consists of two parts: the library for use in other Rust projects and the CLI tool. On this page you'll learn how to use `ardent` on the command-line.

## Commands

### `help`

Once you've completed the [installation](../getting-started/#installation), Ardent should be available in your shell. Running `ardent` without extra arguments will print available sub-commands and flags:

```text
Opinionated formatter for NSIS scripts

Usage: ardent [OPTIONS] [COMMAND]

Commands:
  format  Format NSIS scripts
  check   Check if NSIS scripts are formatted correctly
  help    Print this message or the help of the given subcommand(s)

Options:
  -D, --debug    Print debug messages
  -h, --help     Print help
  -V, --version  Print version
```

### `format`

The `format` sub-command allows the formatting of NSIS scripts. By default, the output is printed to `stdout`. You may use the `--write` flag to apply the formatting changes in-place.

See `ardent format --help` for a list of all options.

### `check`

The `check` sub-command will report whether an NSIS script requires formatting. Using the `--write` flag will apply the changes in-place, while the `--diff` flag will visualize the formatting changes.

See `ardent check --help` for a list of all options.

## Options

:::caution
Ardent is an *opinionated* formatter and the surface to change the default options is intentionally kept small. It's recommended to stick to the defaults.
:::

### `--eol`

Control how line-breaks are represented. Ardent follows the operating system defaults – <abbr title="Carriage Return + Line Feed">CRLF</abbr> on Windows and <abbr title="Line Feed">LF</abbr> elsewhere. Accepts `crlf` or `lf`.

### `--indent-size`

Number of units per indentation level. Defaults to `2`.

### `--use-spaces`

Ardent encourages the use of tabs. While there are often pseudo-religious reasons for choosing tabs or spaces, we prefer tabs for a single practical reason: tabs are preferred by visually impaired programmers, so tabs provide better accessibility. However, you may override it using this flag.

### `--single-quote`

Prefer single quotes instead of double quotes.

### `--comment-style`

Specify whether you want to unify comment markers to `#`or `;`. Block comments are never touched. Accepts `hash` or `semi`.

### `--no-trim`

Ardent will collapse multiple empty lines. Using this flag will prevent this.

### `--print-width`

Ardent uses a print width of 120 characters. This flag allows changing the default value. Setting it to `0` will disable it.
