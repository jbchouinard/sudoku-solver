[![Rust](https://github.com/jbchouinard/sudokusolver/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/jbchouinard/sudokusolver/actions/workflows/rust.yml)

# SudokuSolver

A sudoku solver written in Rust. It can output solution steps as plain HTML.
Only basic strategies are implemented so far so it will only solve fairly easy puzzles.

## Build

```bash
cargo build --release --features html
```

## Usage

Standard output:

```bash
sudokusolver PUZZLE
```

PUZZLE is the sudoku to be solved as a string of 81 digits, where unsolved cells are 0.

HTML output:

```bash```
mkdir html
sudokusolver -o html PUZZLE
```

## Todo
- Rest of basic strategies from SudokuWiki
- Tough strategies from SudokuWiki
- Make strategies configurable
- Highlight logic in solve steps
- Trial and error strategy
- Diabolical strategies from SudokuWiki
- Extreme strategies from SudokuWiki
