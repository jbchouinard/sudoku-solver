use std::fmt;

use crate::{Cell, Grid, Position, Unit};

pub use hidden_single::HiddenSingle;
pub use naked_pair::NakedPair;
pub use naked_triple::NakedTriple;
pub use prune_candidates::PruneCandidates;

mod hidden_single;
mod naked_pair;
mod naked_triple;
mod prune_candidates;

pub fn all_strategies() -> Vec<Strategy> {
    vec![
        Strategy::Cell(Box::new(PruneCandidates)),
        Strategy::Unit(Box::new(HiddenSingle)),
        Strategy::Unit(Box::new(NakedPair)),
        Strategy::Unit(Box::new(NakedTriple)),
    ]
}

pub enum Difficulty {
    Trivial = 0,
    Standard = 1,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Trivial => "Trivial".to_string(),
            Self::Standard => "Standard".to_string(),
        };
        write!(f, "{}", s)
    }
}

pub trait AnyStrategy {
    fn name(&self) -> String;
    fn difficulty(&self) -> Difficulty;
}

pub trait CellStrategy: AnyStrategy {
    fn solve_cell(&self, grid: &Grid, pos: Position) -> Cell;
}

pub trait UnitStrategy: AnyStrategy {
    fn solve_unit(&self, grid: &Grid, unit: &Unit) -> Unit;
}

pub trait GridStrategy: AnyStrategy {
    fn solve_grid(&self, grid: &Grid) -> Grid;
}

pub enum Strategy {
    Cell(Box<dyn CellStrategy>),
    Unit(Box<dyn UnitStrategy>),
    Grid(Box<dyn GridStrategy>),
}

impl Strategy {
    pub fn name(&self) -> String {
        match self {
            Self::Cell(s) => s.name(),
            Self::Unit(s) => s.name(),
            Self::Grid(s) => s.name(),
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        match self {
            Self::Cell(s) => s.difficulty(),
            Self::Unit(s) => s.difficulty(),
            Self::Grid(s) => s.difficulty(),
        }
    }

    pub fn solve(&self, grid: &Grid) -> Grid {
        match self {
            Self::Cell(strategy) => {
                let mut solved_grid = *grid;
                for p in Position::grid_vec() {
                    let solved_cell = strategy.solve_cell(grid, p);
                    solved_grid.set_cell(p, solved_cell);
                }
                solved_grid
            }
            Self::Unit(strategy) => {
                let mut solved_grid = *grid;
                for unit_vec in Position::unit_vecs() {
                    solved_grid.set_cells(strategy.solve_unit(grid, &grid.get_cells(unit_vec)));
                }
                solved_grid
            }
            Self::Grid(strategy) => strategy.solve_grid(grid),
        }
    }
}
