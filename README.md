![Banner](https://s-christy.com/sbs/status-banner.svg?icon=file/difference&hue=200&title=stringdiff&description=A%20Rust%20library%20and%20CLI%20for%20diffing%20strings)

## Overview

<p align="center">
  <img src="./assets/screenshot.png" width=500 />
</p>

`stringdiff` is a Rust library and command-line tool for
computing the difference between two strings. It uses a
longest common substring algorithm to identify added,
removed, and unchanged regions, and can display results with
ANSI color highlighting.

## Features

- Word-level diff via `annotate_strings`
- Character-level diff via `annotate_chars`
- Generic sequence diff via `annotate_sequence` (works on any `PartialEq + Clone` type)
- ANSI color output via `colorize` (green = added, red = removed)
- CLI accepts literal strings, file paths, or stdin (`-`)
- Pure Rust, no dependencies

## Setup

```
cargo build --release
```

## Usage

```
stringdiff <left> <right>
```

Each argument can be:
- A literal string: `"hello world"`
- A file path: `file.txt`
- `-` to read from stdin

## Examples

```
$ stringdiff "foo bar baz" "bar baz qux"
foo bar baz qux

$ stringdiff left.txt right.txt

$ echo "new content" | stringdiff old.txt -
```

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
