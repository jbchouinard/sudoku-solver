use crate::solver::strategies::Strategy;
use crate::Grid;

pub mod strategies;

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Solver {
        Solver { strategies }
    }

    pub fn solve_step(&self, sudoku: &Grid) -> Option<(&dyn Strategy, Grid)> {
        for strat in &self.strategies {
            let solved = strat.solve(&sudoku);
            if &solved != sudoku {
                return Some((strat.as_ref(), solved));
            }
        }
        None
    }

    pub fn solve(&self, sudoku: &Grid) -> Grid {
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
