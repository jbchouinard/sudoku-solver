# SudokuSolver

This is a toy sudoku solver written in  Rust. There arn't any real
strategies implemented yet so this will only solve the most trivial
of puzzles at this point.

## Usage

```bash
sudokusolver FILE_OUT PUZZLE
```

The solution will be written into an HTML file. PUZZLE is the sudoku to be
solved as a string of 81 digits, where unsolved cells are "0".

## Todo

- Easy strategies from SudokuWiki
- Make strategies configurable
- Render suduku to ascii table
- Unit tests
