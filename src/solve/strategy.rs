use crate::SudokuGrid;

pub trait SolveStrategy {
    fn solve(&self, grid: &SudokuGrid) -> SudokuGrid;
    fn name(&self) -> String;
}
