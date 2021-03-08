use super::{AnyStrategy, CellStrategy, Difficulty};
use crate::{Cell, Grid, Position};

pub struct PruneCandidates;

impl AnyStrategy for PruneCandidates {
    fn name(&self) -> String {
        "Prune Candidates".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Trivial
    }
}

impl CellStrategy for PruneCandidates {
    fn solve_cell(&self, grid: &Grid, p: Position) -> Cell {
        match grid.get_cell(p) {
            Cell::Solved(n) => Cell::Solved(n),
            Cell::Unsolved(candidates) => {
                let mut pruned_candidates = candidates;
                for seen in p.seen_vec(false) {
                    if let Cell::Solved(n) = grid.get_cell(seen) {
                        pruned_candidates.remove(&n);
                    }
                }
                Cell::Unsolved(pruned_candidates)
            }
        }
    }
}
