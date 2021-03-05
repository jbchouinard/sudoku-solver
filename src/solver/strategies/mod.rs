pub mod hidden_single;
pub mod prune_candidates;

use crate::Grid;

pub use hidden_single::HiddenSingle;
pub use prune_candidates::PruneCandidates;

pub trait Strategy {
    fn solve(&self, grid: &Grid) -> Grid;
    fn name(&self) -> String;
}

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(PruneCandidates), Box::new(HiddenSingle)]
}
