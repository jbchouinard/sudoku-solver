use std::collections::HashSet;

use super::sets::{Order, Subsets};
use super::{Difficulty, StrategyDelta, UnitStrategy};
use crate::{Cell, CellValue, Position, Unit};

#[derive(Clone)]
pub struct HiddenN<const N: usize>;

impl<const N: usize> UnitStrategy for HiddenN<N> {
    fn name(&self) -> String {
        format!("Hidden {}", Order(N))
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }

    fn solve_unit(&self, unit: &Unit) -> StrategyDelta {
        let mut delta = StrategyDelta::new();
        let mut pmap = PositionMap::new();
        for (p, cell) in unit {
            pmap.add_cell(p, cell);
        }
        for (hidden_vs, positions) in pmap.find_hidden(N) {
            for p in positions {
                if let Some(Cell::Unsolved(candidates)) = unit.get(&p) {
                    for candidate in candidates.to_vec() {
                        if !hidden_vs.contains(&candidate) {
                            delta.eliminate(p, candidate);
                        }
                    }
                }
            }
        }
        delta
    }
}

pub struct PositionMap(Subsets<CellValue, Position>);

impl PositionMap {
    pub fn new() -> Self {
        Self(Subsets::new())
    }

    pub fn add_cell(&mut self, pos: &Position, cell: &Cell) {
        if let Cell::Unsolved(candidates) = cell {
            for cv in candidates.to_vec() {
                self.0.insert(cv, *pos);
            }
        }
    }

    pub fn find_hidden(&self, n: usize) -> Vec<(HashSet<CellValue>, HashSet<Position>)> {
        self.0.find_critical_sets(n)
    }
}
