use super::{Difficulty, StrategyDelta, UnitStrategy};
use crate::{Candidates, Cell, Grid, Unit};

#[derive(Clone)]
pub struct PruneCandidates;

impl UnitStrategy for PruneCandidates {
    fn name(&self) -> String {
        "Prune Candidates".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Trivial
    }

    fn solve_unit(&self, _grid: &Grid, unit: &Unit) -> StrategyDelta {
        let mut delta = StrategyDelta::new();
        let mut to_prune = Candidates::new([false; 9]);
        for cell in unit.values() {
            if let Cell::Solved(n) = cell {
                to_prune.add(&n);
            }
        }
        for (p, cell) in unit {
            if let Cell::Unsolved(candidates) = cell {
                for n in to_prune.to_vec() {
                    if candidates.can_be(&n) {
                        delta.eliminate(*p, n);
                    }
                }
            }
        }
        delta
    }
}
