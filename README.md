[![Rust](https://github.com/jbchouinard/sudokusolver/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/jbchouinard/sudokusolver/actions/workflows/rust.yml)

# SudokuSolver

A sudoku solver written in Rust for fun. It can output solution steps as plain HTML.
Only basic strategies are implemented so far so it will only mostly solve easy to medium puzzles.

The strategies are inspired by the [SudokuWiki Solver](https://www.sudokuwiki.org/sudoku.htm) documentation.

## Build

This project depends on features only on nightly toolchain.

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

```bash
mkdir html
sudokusolver -o html PUZZLE
```
