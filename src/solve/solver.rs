use crate::solve::strategies::{CheckSolvedStrategy, PruneCandidatesStrategy};
use crate::solve::strategy::SolveStrategy;
use crate::SudokuGrid;

pub fn all_strategies() -> Vec<Box<dyn SolveStrategy>> {
    let mut strats: Vec<Box<dyn SolveStrategy>> = Vec::new();
    strats.push(Box::new(CheckSolvedStrategy));
    strats.push(Box::new(PruneCandidatesStrategy));
    strats
}

pub struct SudokuSolver {
    strategies: Vec<Box<dyn SolveStrategy>>,
}

impl SudokuSolver {
    pub fn new(strategies: Vec<Box<dyn SolveStrategy>>) -> SudokuSolver {
        SudokuSolver { strategies }
    }

    pub fn solve_step(&self, sudoku: &SudokuGrid) -> Option<(&dyn SolveStrategy, SudokuGrid)> {
        for strat in &self.strategies {
            let solved = strat.solve(&sudoku);
            if &solved != sudoku {
                return Some((strat.as_ref(), solved));
            }
        }
        None
    }

    pub fn solve(&self, sudoku: &SudokuGrid) -> SudokuGrid {
        let mut solution = sudoku.clone();
        loop {
            match self.solve_step(&solution) {
                Some((_, sol)) => {
                    solution = sol;
                }
                None => {
                    break;
                }
            }
        }
        solution
    }
}
