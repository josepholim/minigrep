# minigrep

A simple, fast, and flexible grep clone written in Rust.

**minigrep** supports literal, case-insensitive, regex, and fuzzy matching, multiple patterns, colorized output, and more.

---

## Features

* **Multiple Patterns** (`-e, --query`): Search for one or more patterns in a single run (e.g., `-e foo -e bar`).
* **Literal & Case-Insensitive**: Default literal matching or ignore-case with `-i, --ignore-case`.
* **Regex Matching** (`-r, --regex`): Full regex support via `regex` crate, with `--ignore-case` option.
* **Fuzzy Matching** (`--fuzzy`): Fuzzy search powered by `SkimMatcherV2`, tunable with `--fuzzy-threshold`.
* **Invert Match** (`-v, --invert`): Show lines that **do not** match.
* **Context Lines** (`-C, -A, -B`): Show lines around each match.
* **Colorized Output** (`--color`): Highlight matches with ANSI colors.
* **Recursive Search** (`-r, --recursive`, `--ignore-file`): Traverse directories, search all files, and optionally ignore some files.
* **Count & Show Line Numbers Modes** (`-c, --count`, `-n, --show-line-numbers`): Show match counts or show line numbers.

---

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed (>=1.60).

```bash
# Clone the repo
git clone https://github.com/josepholim/minigrep.git
cd minigrep

# Build the binary
cargo build --release
```

---

## Usage

```text
Usage: minigrep [OPTIONS] <PATHS>...

Search for patterns in files or directories.

Arguments:
  <PATHS>...        File or directory paths to search

Options:
  -e, --query <PATTERN>...      Pattern(s) to search for (can use multiple)
      --path <PATH>             Search in the <PATH> file or directory
  -i, --ignore-case             Case-insensitive search
  -E, --regex                   Treat patterns as regular expressions
      --fuzzy                   Enable fuzzy matching
      --fuzzy-threshold <SCORE> Minimum score threshold for fuzzy (default: 1)
  -v, --invert                  Invert match (select non-matching lines)
  -C, --context <NUM>           Show <NUM> lines before and after matches
  -A, --context-after <NUM>     Show <NUM> lines after matches
  -B, --context-before <NUM>    Show <NUM> lines before matches
  -n, --show-line-numbers       Prefix each match with its line number
  -c, --count                   Print only a count of matching lines per file
      --color                   Enable ANSI color highlighting
  -R, --recursive               Recurse into directories
      --ignore-file <FILE>      Specify an ignore file <FILE> to skip files (default: .gitignore)
  -h, --help                    Print help information
  -V, --version                 Print version information
```

---

## Examples

Search recursively for "TODO" or "FIXME" in the current directory, case-insensitive with color:

```bash
minigrep -R -i --color -e TODO -e FIXME --path .
```

Show 2 lines of context around each regex match in `src/`:

```bash
minigrep -R -E -C 2 -e "fn\s+\w+" --path src/
```

Count lines not matching "error" in log files:

```bash
minigrep -R -e error -v -c --path logs/
```

Fuzzy search across multiple patterns:

```bash
minigrep -R --fuzzy --fuzzy-threshold 150 -e init -e config --path src/
```
