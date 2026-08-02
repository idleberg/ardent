# Plan: Add `--diff` to `check` in both CLIs

## Shared behaviour spec (applies to both)

| Aspect | Decision |
|---|---|
| Flag | `-d, --diff` on the `check` subcommand only |
| Mutually exclusive with `--write`? | No — they can coexist: `--write` fixes, `--diff` shows what was fixed. If both are given, apply the fix *and* print the diff |
| Mutually exclusive with `--silent`? | Yes — `--diff` output is suppressed when `--silent` is active |
| Stdin | Print diff to stdout; omit the filename header line |
| No drift | Print nothing (same as today) |
| Diff format | Unified diff, `--- a/<file>` / `+++ b/<file>` headers (no timestamps), coloured `+`/`-` lines when stdout is a TTY |
| Context lines | 3 lines before and after each hunk (GNU `diff -u` default) |
| Exit code | Unchanged — still exits 1 on drift |

### Header format (both CLIs must match exactly)

```
--- a/<filename>
+++ b/<filename>
@@ -n,m +n,m @@
```

No timestamps. Git-style. Both implementations must pin this explicitly — do not rely on library defaults.

### Dependency decision

Prefer **no new dependency**. A unified diff is ~30–50 lines to implement inline since `check()` already returns both the original and formatted strings. Avoids bundle size, transitive deps, licence, and maintenance concerns.

---

## 1 · Rust CLI (`ardent` — `idleberg/ardent`)

### 1a. `src/main.rs` — `Commands::Check`

Add `diff: bool` to the `Check` variant:

```rust
#[arg(short = 'd', long, help = "Print a unified diff for each file with issues")]
diff: bool,
```

Pass it through `run_check(…, diff, …)`.

### 1b. `run_check` function

- Add `diff: bool` parameter.
- After `formatter.check(raw)` returns `Some(formatted)`:
  - Call a new helper `print_diff(filename, raw, &formatted)`.
  - Only call it when `!silent` (already tracked via the `SILENT` atomic).

### 1c. New helper `print_diff`

Implement inline (~30–50 lines) without a new dependency:

```rust
fn print_diff(label: &str, original: &str, formatted: &str) {
    // print --- a/<label> / +++ b/<label> header (no timestamp)
    // compute hunks with 3-line context
    // colour +/- lines via existing colour helpers when stdout is a TTY
}
```

Colour: reuse existing `dim()` / `blue()` helpers; add red/green only when `std::io::stdout().is_terminal()`.

### 1d. `main()` dispatch

```rust
Some(Commands::Check { files, write, silent, diff, formatting }) => {
    …
    run_check(&files, write, diff, &formatting, cli.debug)
}
```

### 1e. Tests

- Add a unit test that calls `run_check` on a fixture pair with `diff=true` and asserts the output contains `--- a/`, `+++ b/`, and `-`/`+` lines.
- Assert `--diff` + `--silent` → diff is **not** printed.

---

## 2 · Node.js CLI (`dent` — `idleberg/nsis-org`, `packages/dent-cli`)

### 2a. `src/commands/check.ts` — `checkCommand()`

Add the option to the command definition:

```ts
cmd.option('-d, --diff', 'print a unified diff for each file with issues', false);
```

Extend `CheckOptions`:

```ts
type CheckOptions = SharedOptions & { write: boolean; silent: boolean; diff: boolean };
```

### 2b. `runCheck` — stdin path

After `result !== null`, before `process.exit(1)`:

```ts
if (options.diff) {
    printDiff('<stdin>', rawContents, result);
}
```

### 2c. `runCheck` — file path

In the `processFiles` callback, after `drifted.push(file)`, before the `options.write` branch:

```ts
if (options.diff) {
    printDiff(file, _rawContents, result);
}
```

Note: `_rawContents` (4th callback param) needs to be un-prefixed to use it here.

### 2d. New helper `printDiff`

Implement inline without a new dependency:

```ts
function printDiff(label: string, original: string, formatted: string): void {
    // emit --- a/<label> / +++ b/<label> (no timestamp)
    // compute hunks with 3-line context
    // colour +/-/@@ lines via kleur/colors (already a dependency)
    logger.log(output);
}
```

Colour with already-present `kleur/colors`. Suppress colour when stdout is not a TTY.

### 2e. Tests (`src/commands/check.test.ts`)

- Add `'exposes --diff option'` to the shape suite (alongside `--write`).
- Stdin scenario: unformatted input + `--diff` → `logger.log` called with `--- a/`, `+++ b/`, `-`/`+` lines.
- File scenario: unformatted file + `--diff` → same assertion.
- Assert `--diff` + `--silent` → diff is **not** printed.

---

## 3 · Documentation

| File | Change |
|---|---|
| `ardent/README.md` | Add `--diff` to the `check` usage example and option table |
| `dent-cli/README.md` | Add `--diff` to the `check` options block in the help output snippet |

---

## Implementation order

1. Agree on `--write` + `--diff` coexistence behaviour (fix-and-show vs. fix-silently).
2. Rust: implement `print_diff` inline, wire up flag, add tests.
3. Node: implement `printDiff` inline, wire up flag, add tests.
4. Update both READMEs.