use crate::solve::strategies::Strategy;
use crate::{Cell, Grid};

pub struct PruneCandidates;

impl PruneCandidates {
    fn solve_cell(&self, grid: &Grid, x: u8, y: u8) -> Option<Cell> {
        let cell = grid.get_cell(x, y);
        match cell {
            Cell::Solved(_) => None,
            Cell::Unsolved(mut candidates) => {
                for j in 1..=9 {
                    if let Cell::Solved(n) = grid.get_cell(x, j) {
                        let p: usize = (n - 1).into();
                        candidates[p] = false;
                    }
                }
                for i in 1..=9 {
                    if let Cell::Solved(n) = grid.get_cell(i, y) {
                        let p: usize = (n - 1).into();
                        candidates[p] = false;
                    }
                }
                for j in Grid::box_range(y) {
                    for i in Grid::box_range(x) {
                        if let Cell::Solved(n) = grid.get_cell(i, j) {
                            let p: usize = (n - 1).into();
                            candidates[p] = false;
                        }
                    }
                }
                Some(Cell::Unsolved(candidates))
            }
        }
    }
}

impl Strategy for PruneCandidates {
    fn solve(&self, grid: &Grid) -> Grid {
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
        "[Trivial] Prune Candidates".to_string()
    }
}

pub struct PromoteUniqueCandidate;

impl Strategy for PromoteUniqueCandidate {
    fn solve(&self, grid: &Grid) -> Grid {
        let mut new_grid = grid.clone();
        for y in 1..=9 {
            for x in 1..=9 {
                if let Some(candidates) = grid.get_cell(x, y).candidates() {
                    if candidates.len() == 1 {
                        new_grid.set_cell(x, y, Cell::Solved(candidates[0]));
                    }
                }
            }
        }
        new_grid
    }
    fn name(&self) -> String {
        "[Trivial] Promote Unique Candidates".to_string()
    }
}
