pub mod trivial;

use crate::Grid;

pub trait Strategy {
    fn solve(&self, grid: &Grid) -> Grid;
    fn name(&self) -> String;
}

use trivial::PromoteUniqueCandidate;
use trivial::PruneCandidates;

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(PromoteUniqueCandidate), Box::new(PruneCandidates)]
}
