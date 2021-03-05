# SudokuSolver

This is a toy sudoku solver written in  Rust. There arn't many
strategies implemented yet so this will only solve pretty easy
puzzles.

## Usage

```bash
mkdir html
sudokusolver -o html PUZZLE
```

The solution steps will be written to html/. PUZZLE is the sudoku to be
solved as a string of 81 digits, where unsolved cells are "0".

## Todo
- Unit tests
- Easy strategies from SudokuWiki
- Make strategies configurable
- Highlight logic in solve steps
- Tough strategies from SudokuWiki
- Trial and error strategy
- Diabolical strategies from SudokuWiki
- Extreme strategies from SudokuWiki
