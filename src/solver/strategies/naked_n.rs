use std::collections::HashSet;

use super::sets::{Order, Subsets};
use super::{Difficulty, StrategyDelta, UnitStrategy};
use crate::{Cell, CellValue, Position, Unit};

#[derive(Clone)]
pub struct NakedN<const N: usize>;

impl<const N: usize> UnitStrategy for NakedN<N> {
    fn name(&self) -> String {
        format!("Naked {}", Order(N))
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }

    fn solve_unit(&self, unit: &Unit) -> StrategyDelta {
        let mut delta = StrategyDelta::new();
        let mut cmap = CandidateMap::new();
        for (p, cell) in unit {
            cmap.add_cell(p, cell);
        }
        for (naked_ps, naked_vs) in cmap.find_naked(N) {
            for (p, cell) in unit {
                if naked_ps.contains(p) {
                    continue;
                }
                if let Some(candidates) = cell.candidates() {
                    for candidate in candidates.to_vec() {
                        if naked_vs.contains(&candidate) {
                            delta.eliminate(*p, candidate);
                        }
                    }
                }
            }
        }
        delta
    }
}

pub struct CandidateMap(Subsets<Position, CellValue>);

impl CandidateMap {
    pub fn new() -> Self {
        Self(Subsets::new())
    }

    pub fn add_cell(&mut self, pos: &Position, cell: &Cell) {
        match cell {
            Cell::Unsolved(candidates) => {
                for cv in candidates.to_vec() {
                    self.0.insert(*pos, cv);
                }
            }
            Cell::Solved(cv) => {
                self.0.insert(*pos, *cv);
            }
        }
    }

    pub fn find_naked(&self, n: usize) -> Vec<(HashSet<Position>, HashSet<CellValue>)> {
        self.0.find_critical_sets(n)
    }
}
