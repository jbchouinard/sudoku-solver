use std::collections::HashMap;
use std::fmt;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{Candidates, Cell, CellValue, Grid, Position, Unit};

mod hidden_n;
mod naked_n;
mod sets;

use hidden_n::HiddenN;
use naked_n::NakedN;

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(UnitStrategyWrapper(NakedN::<1>)),
        Box::new(UnitStrategyWrapper(HiddenN::<1>)),
        Box::new(UnitStrategyWrapper(NakedN::<2>)),
        Box::new(UnitStrategyWrapper(HiddenN::<2>)),
        Box::new(UnitStrategyWrapper(NakedN::<3>)),
        Box::new(UnitStrategyWrapper(HiddenN::<3>)),
        Box::new(UnitStrategyWrapper(NakedN::<4>)),
        Box::new(UnitStrategyWrapper(HiddenN::<4>)),
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

    // For non-overlapping positions only!
    pub fn extend(&mut self, other: StrategyDelta) {
        self.solve.extend(other.solve);
        self.eliminate.extend(other.eliminate);
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
        write!(f, "{}", parts.join(", "))
    }
}

/// A strategy that operates on the whole Sudoku grid.
pub trait Strategy: DynClone {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
    fn solve(&self, grid: &Grid) -> StrategyDelta;
}

clone_trait_object!(Strategy);

/// A strategy that operates on a unit (row, column or box).
pub trait UnitStrategy {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
    fn solve_unit(&self, unit: &Unit) -> StrategyDelta;
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
        let mut delta = StrategyDelta::new();
        for vec in Position::row_vecs() {
            let step_delta = self.0.solve_unit(&grid.get_cells(vec));
            delta.extend(step_delta);
        }
        if !delta.is_empty() {
            return delta;
        }
        for vec in Position::col_vecs() {
            let step_delta = self.0.solve_unit(&grid.get_cells(vec));
            delta.extend(step_delta);
        }
        if !delta.is_empty() {
            return delta;
        }
        for vec in Position::box_vecs() {
            let step_delta = self.0.solve_unit(&grid.get_cells(vec));
            delta.extend(step_delta);
        }
        if !delta.is_empty() {
            return delta;
        }
        delta
    }
}
