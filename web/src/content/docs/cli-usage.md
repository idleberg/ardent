---
title: Command-line usage
description: Learn how to use the Ardent CLI tool.
---

The Rust crate for Ardent consists of two parts: the library for use in other Rust projects and the CLI tool. On this page you'll learn how to use `ardent` on the command-line.

## Commands

### `help`

Once you've completed the [installation](../getting-started/#installation), Ardent should be available in your shell. Running `ardent` without extra arguments will print available sub-commands and flags:

```text
🍁 Ardent is an opinionated code formatter for NSIS scripts

Usage: ardent [OPTIONS] [COMMAND]

Commands:
  format       Command to format NSIS scripts
  check        Command to check if NSIS scripts are formatted correctly
  completions  Command to print a shell completion script
  help         Print this message or the help of the given subcommand(s)

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

### `completions`

The `completions` sub-command prints a shell completion script to `stdout`, so that pressing <kbd>Tab</kbd> offers Ardent's sub-commands and flags. It accepts `bash`, `elvish`, `fish`, `powershell` or `zsh`.

:::note
Completion scripts are generated from the command-line interface as it is at the time you run the command. Re-run it after upgrading Ardent, otherwise new sub-commands and flags won't be offered.
:::

Ardent only prints the script — where it belongs is up to your shell, so redirect it accordingly.

**Powershell**

```powershell
New-Item -ItemType File -Path $PROFILE -Force | Out-Null
ardent completions powershell | Out-File -Append -Encoding utf8 $PROFILE
```

**Bash**

```shell
# bash, which requires bash-completion to be installed
ardent completions bash > ~/.local/share/bash-completion/completions/ardent
```

**Fish**

```shell
# fish, which needs no further setup
ardent completions fish > ~/.config/fish/completions/ardent.fish
```

Start a new shell session for any of the above to take effect.

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

Specify whether you want to unify comment markers to `#` or `;`. Block comments are never touched. Accepts `hash` or `semi`.

### `--no-trim`

Ardent will collapse multiple empty lines. Using this flag will prevent this.

### `--print-width`

Ardent uses a print width of 120 characters. This flag allows changing the default value. Setting it to `0` will disable it.
