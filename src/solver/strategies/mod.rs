use std::collections::HashMap;
use std::fmt;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{Candidates, Cell, CellValue, Grid, Position, Unit};

pub use hidden_single::HiddenSingle;
pub use naked_pair::NakedPair;
pub use naked_triple::NakedTriple;
pub use prune_candidates::PruneCandidates;

mod hidden_single;
mod naked_pair;
mod naked_triple;
mod prune_candidates;

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        // PruneCandidates must always run first, because other strategies
        // assume that candidates are in a consistent state with solved cells,
        // which is only ensured by running PruneCandidates
        Box::new(UnitStrategyWrapper(PruneCandidates)),
        Box::new(UnitStrategyWrapper(HiddenSingle)),
        Box::new(UnitStrategyWrapper(NakedPair)),
        Box::new(UnitStrategyWrapper(NakedTriple)),
    ]
}

pub enum Difficulty {
    Trivial = 0,
    Standard = 1,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trivial => "Trivial",
                Self::Standard => "Standard",
            }
        )
    }
}

pub enum StrategyResult {
    // Indicates that the strategy was able to solve part of the grid
    Success,
    Failure,
}

pub struct StrategyDelta {
    solve: HashMap<Position, CellValue>,
    eliminate: HashMap<Position, Candidates>,
}

impl StrategyDelta {
    pub fn new() -> Self {
        StrategyDelta {
            solve: HashMap::new(),
            eliminate: HashMap::new(),
        }
    }

    pub fn result(&self) -> StrategyResult {
        if self.is_empty() {
            StrategyResult::Failure
        } else {
            StrategyResult::Success
        }
    }
    pub fn is_empty(&self) -> bool {
        self.solve.is_empty() && self.eliminate.is_empty()
    }

    pub fn solve(&mut self, pos: Position, v: CellValue) {
        self.solve.insert(pos, v);
    }

    pub fn eliminate(&mut self, pos: Position, v: CellValue) {
        self.eliminate
            .entry(pos)
            .or_insert_with(|| Candidates::new([false; 9]))
            .add(&v);
    }

    pub fn apply(&self, grid: &mut Grid) {
        for (p, val) in &self.solve {
            grid.set_cell(*p, Cell::Solved(*val));
        }
        for (p, to_prune) in &self.eliminate {
            if let Cell::Unsolved(candidates) = grid.get_cell(*p) {
                let mut pruned = candidates;
                for v in to_prune.to_vec() {
                    pruned.remove(&v);
                }
                grid.set_cell(*p, Cell::Unsolved(pruned));
            }
        }
    }
}

impl Default for StrategyDelta {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StrategyDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let mut parts = vec![];
        for (p, v) in &self.solve {
            let n: u8 = (*v).into();
            parts.push(format!("{}={}", p, n));
        }
        for (p, cdx) in &self.eliminate {
            parts.push(format!("{}-{}", p, cdx));
        }
        write!(f, "{}", parts.join(","))
    }
}

pub trait Strategy: DynClone {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
    fn solve(&self, grid: &Grid) -> StrategyDelta;
}

clone_trait_object!(Strategy);

pub trait CellStrategy {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
    fn solve_cell(&self, grid: &Grid, pos: Position) -> StrategyDelta;
}

#[derive(Clone)]
struct CellStrategyWrapper<T: CellStrategy>(T);

impl<T: CellStrategy + Clone> Strategy for CellStrategyWrapper<T> {
    fn name(&self) -> String {
        self.0.name()
    }
    fn difficulty(&self) -> Difficulty {
        self.0.difficulty()
    }
    fn solve(&self, grid: &Grid) -> StrategyDelta {
        for p in Position::grid_vec() {
            // If the cell changed, candidates may be in an
            // inconsistent state; break so that PruneCandidates is re-ran
            // before changing anything else.
            let delta = self.0.solve_cell(&grid, p);
            if !delta.is_empty() {
                return delta;
            }
        }
        StrategyDelta::new()
    }
}

pub trait UnitStrategy {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
    fn solve_unit(&self, grid: &Grid, unit: &Unit) -> StrategyDelta;
}

#[derive(Clone)]
struct UnitStrategyWrapper<T: UnitStrategy>(T);

impl<T: UnitStrategy + Clone> Strategy for UnitStrategyWrapper<T> {
    fn name(&self) -> String {
        self.0.name()
    }
    fn difficulty(&self) -> Difficulty {
        self.0.difficulty()
    }
    fn solve(&self, grid: &Grid) -> StrategyDelta {
        for unit_vec in Position::unit_vecs() {
            let delta = self.0.solve_unit(&grid, &grid.get_cells(unit_vec));
            // If any of the cells changed, candidates may be in an
            // inconsistent state; break so that PruneCandidates is re-ran
            // before changing anything else.
            if !delta.is_empty() {
                return delta;
            }
        }
        StrategyDelta::new()
    }
}
