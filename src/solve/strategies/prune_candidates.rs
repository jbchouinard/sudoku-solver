use crate::solve::strategy::SolveStrategy;
use crate::{SudokuCell, SudokuGrid};

pub struct PruneCandidatesStrategy;

impl PruneCandidatesStrategy {
    fn solve_cell(&self, grid: &SudokuGrid, x: u8, y: u8) -> Option<SudokuCell> {
        let cell = grid.get_cell(x, y);
        match cell {
            SudokuCell::Solved(_) => None,
            SudokuCell::Unsolved(mut candidates) => {
                for j in 1..=9 {
                    if let SudokuCell::Solved(n) = grid.get_cell(x, j) {
                        let p: usize = (n - 1).into();
                        candidates[p] = false;
                    }
                }
                for i in 1..=9 {
                    if let SudokuCell::Solved(n) = grid.get_cell(i, y) {
                        let p: usize = (n - 1).into();
                        candidates[p] = false;
                    }
                }
                for j in SudokuGrid::box_range(y) {
                    for i in SudokuGrid::box_range(x) {
                        if let SudokuCell::Solved(n) = grid.get_cell(i, j) {
                            let p: usize = (n - 1).into();
                            candidates[p] = false;
                        }
                    }
                }
                Some(SudokuCell::Unsolved(candidates))
            }
        }
    }
}

impl SolveStrategy for PruneCandidatesStrategy {
    fn solve(&self, grid: &SudokuGrid) -> SudokuGrid {
        let mut new_grid = grid.clone();
        for i in 1..=9 {
            for j in 1..=9 {
                if let Some(cell) = self.solve_cell(grid, i, j) {
                    new_grid.set_cell(i, j, cell);
                }
            }
        }
        new_grid
    }
    fn name(&self) -> String {
        "PruneCandidates".to_string()
    }
}
