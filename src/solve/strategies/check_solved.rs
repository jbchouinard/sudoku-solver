use crate::solve::strategy::SolveStrategy;
use crate::{SudokuCell, SudokuGrid};

pub struct CheckSolvedStrategy;

impl SolveStrategy for CheckSolvedStrategy {
    fn solve(&self, grid: &SudokuGrid) -> SudokuGrid {
        let mut new_grid = grid.clone();
        for y in 1..=9 {
            for x in 1..=9 {
                if let Some(candidates) = grid.get_cell(x, y).candidates() {
                    if candidates.len() == 1 {
                        new_grid.set_cell(x, y, SudokuCell::Solved(candidates[0]));
                    }
                }
            }
        }
        new_grid
    }
    fn name(&self) -> String {
        "CheckSolved".to_string()
    }
}
