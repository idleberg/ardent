---
title: Formatting
description: The formatting rules Ardent always applies to your NSIS scripts.
---

The [command-line usage](../cli-usage/) page covers what you can configure: quotes, indentation and print width. This page covers the rest — the *opinionated* part, the rules Ardent always applies.

## Rules

### Casing

NSIS is case-insensitive, so `OutFile`, `outfile` and `OUTFILE` all work. Ardent settles on one spelling for you — the canonical one — for instructions, parameters, built-in variables and built-in defines. Names you define yourself keep the casing you gave them.

```diff live lang="nsis"
- outFile "demo.exe"
- NAME "Screaming Installer"
+ OutFile "demo.exe"
+ Name "Screaming Installer"

Section
-  detailprint "NSIS is installed at ${nsisdir}"
+  DetailPrint "NSIS is installed at ${NSISDIR}"
SectionEnd
```

### Indentation

Ardent indents the body of every block and removes indentation that doesn't belong to one. Blocks nest, so a conditional inside a section is indented twice.

```diff live lang="nsis"
OutFile "demo.exe"
-  Name "Indented Installer"
-   Unicode true
+Name "Indented Installer"
+Unicode true

Section
-!ifdef CONDITION
-DetailPrint "Are we indented?"
-!endif
+  !ifdef CONDITION
+    DetailPrint "Are we indented?"
+  !endif
SectionEnd
```

### Whitespace

Ardent separates blocks with a blank line and collapses longer runs of blank lines into one.

```diff live lang="nsis"
OutFile "demo.exe"
-# one lonely section
-Section
-SectionEnd
-Function .onInit
-FunctionEnd
+
+# one lonely section
+Section
+SectionEnd
+
+Function .onInit
+FunctionEnd
-
-
-

# trim all that whitespace
Function .onAbort
FunctionEnd
```
