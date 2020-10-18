use sudoku::SudokuGrid;

fn main() {
    let grid = SudokuGrid::from_chars(&['2'; 81]);
    println!("{}", grid.to_html());
}
