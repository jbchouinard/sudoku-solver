use super::{AnyStrategy, Difficulty, UnitStrategy};
use crate::{Cell, CellValue, Grid, Position, Unit};

pub struct HiddenSingle;

impl AnyStrategy for HiddenSingle {
    fn name(&self) -> String {
        "Hidden Single".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }
}

impl UnitStrategy for HiddenSingle {
    fn solve_unit(&self, _grid: &Grid, unit: &Unit) -> Unit {
        let mut solved_unit = Unit::new();
        let mut map = CandidateMap::new();
        for (p, cell) in unit {
            map.add_cell(p, cell);
        }
        for n in 1..=9 {
            let positions = map.positions(n.into());
            if positions.len() == 1 {
                let p = positions[0];
                solved_unit.insert(p, Cell::Solved(n.into()));
            }
        }
        solved_unit
    }
}

struct CandidateMap([Vec<Position>; 9]);

impl CandidateMap {
    pub fn new() -> Self {
        CandidateMap([
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ])
    }
    pub fn add_cell(&mut self, pos: &Position, cell: &Cell) {
        if let Cell::Unsolved(candidates) = cell {
            for val in candidates.to_vec() {
                let n: usize = val.into();
                self.0[n - 1].push(*pos);
            }
        }
    }
    pub fn positions(&self, value: CellValue) -> &Vec<Position> {
        let n: usize = value.into();
        &self.0[n - 1]
    }
}
