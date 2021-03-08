use std::time::{Duration, Instant};

use super::Grid;
use strategies::{Strategy, StrategyDelta, StrategyResult};

pub mod strategies;

#[cfg(test)]
mod tests;

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Solver {
        Solver { strategies }
    }

    pub fn solve_step(&self, grid: &Grid) -> Option<SolutionStep> {
        let t_start = Instant::now();
        for strategy in &self.strategies {
            let delta = strategy.solve(grid);
            let t_elapsed = t_start.elapsed();
            if let StrategyResult::Success = delta.result() {
                return Some(SolutionStep {
                    strategy: strategy.clone(),
                    delta,
                    time: t_elapsed,
                });
            }
        }
        None
    }

    pub fn solve(&self, grid: &mut Grid) -> Vec<SolutionStep> {
        let mut steps = vec![];
        while let Some(step) = self.solve_step(grid) {
            step.delta.apply(grid);
            steps.push(step);
        }
        steps
    }
}

pub struct SolutionStep {
    pub strategy: Box<dyn Strategy>,
    pub delta: StrategyDelta,
    pub time: Duration,
}
