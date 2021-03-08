use std::time::{Duration, Instant};

use super::{Grid, GridSolvedDiff};
use strategies::Strategy;

pub mod strategies;

#[cfg(test)]
mod tests;

pub struct Solver {
    strategies: Vec<Strategy>,
}

impl Solver {
    pub fn new(strategies: Vec<Strategy>) -> Solver {
        Solver { strategies }
    }

    pub fn solve_step(&self, sudoku: &Grid) -> Option<(&Strategy, Grid, Duration)> {
        let start = Instant::now();
        for strat in &self.strategies {
            let solved = strat.solve(&sudoku);
            if &solved != sudoku {
                let elapsed = start.elapsed();
                return Some((strat, solved, elapsed));
            }
        }
        None
    }

    pub fn solve(&self, sudoku: &Grid) -> Solution {
        let mut current = *sudoku;
        let mut steps = vec![];
        while let Some((strategy, solved, duration)) = self.solve_step(&current) {
            steps.push(SolutionStep {
                strategy,
                solved_diff: current.solved_diff_from(&solved),
                time: duration,
            });
            current = solved;
        }
        Solution {
            grid: current,
            steps,
        }
    }
}

pub struct Solution<'a> {
    pub grid: Grid,
    pub steps: Vec<SolutionStep<'a>>,
}

pub struct SolutionStep<'a> {
    pub strategy: &'a Strategy,
    pub solved_diff: GridSolvedDiff,
    pub time: Duration,
}
