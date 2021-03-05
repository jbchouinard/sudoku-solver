use crate::solve::strategies::Strategy;
use crate::{Cell, Grid, Position};

pub struct PruneCandidates;

impl Strategy for PruneCandidates {
    fn solve(&self, grid: &Grid) -> Grid {
        let mut new_grid = grid.clone();
        for p in Position::iter_grid() {
            new_grid.set_cell(
                p,
                match grid.get_cell(p) {
                    Cell::Solved(n) => Cell::Solved(n.clone()),
                    Cell::Unsolved(candidates) => {
                        let mut pruned = candidates.clone();
                        for seen in p.iter_seen(false) {
                            if let Cell::Solved(v) = grid.get_cell(seen) {
                                pruned.remove(v);
                            }
                        }
                        Cell::Unsolved(pruned)
                    }
                },
            )
        }
        new_grid
    }
    fn name(&self) -> String {
        "[Trivial] Prune Candidates".to_string()
    }
}
