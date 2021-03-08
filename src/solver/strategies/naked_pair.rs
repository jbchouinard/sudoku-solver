use std::collections::HashMap;
use std::convert::TryInto;

use super::{Difficulty, StrategyDelta, UnitStrategy};
use crate::{Cell, CellValue, Grid, Position, Unit};

#[derive(Clone)]
pub struct NakedPair;

impl UnitStrategy for NakedPair {
    fn name(&self) -> String {
        "Naked Pair".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }

    fn solve_unit(&self, _grid: &Grid, unit: &Unit) -> StrategyDelta {
        let mut delta = StrategyDelta::new();
        let mut pairmap = CandidatePairMap::new();
        for (p, cell) in unit {
            pairmap.add_cell(p, cell);
        }
        for (pair, pair_p) in pairmap.map {
            // If there are two cells with the exact same pair of candidates
            if pair_p.len() == 2 {
                // Eliminate the candidates from all other cells in the unit
                for (other_p, other) in unit {
                    if other_p != &pair_p[0] && other_p != &pair_p[1] {
                        if let Cell::Unsolved(candidates) = other {
                            if candidates.can_be(&pair[0]) {
                                delta.eliminate(*other_p, pair[0]);
                            }
                            if candidates.can_be(&pair[1]) {
                                delta.eliminate(*other_p, pair[1]);
                            }
                        }
                    }
                }
            }
        }
        delta
    }
}

struct CandidatePairMap {
    pub map: HashMap<[CellValue; 2], Vec<Position>>,
}

impl CandidatePairMap {
    pub fn new() -> Self {
        CandidatePairMap {
            map: HashMap::new(),
        }
    }

    pub fn add_cell(&mut self, pos: &Position, cell: &Cell) {
        if let Cell::Unsolved(candidates) = cell {
            let cd_vec = candidates.to_vec();
            if cd_vec.len() == 2 {
                let cd_arr: [CellValue; 2] = cd_vec.try_into().unwrap();
                match self.map.get_mut(&cd_arr) {
                    Some(positions) => {
                        positions.push(*pos);
                    }
                    None => {
                        self.map.insert(cd_arr, vec![*pos]);
                    }
                }
            }
        }
    }
}
