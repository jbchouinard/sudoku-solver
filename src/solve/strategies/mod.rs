pub mod easy;
pub mod trivial;

use crate::Grid;

pub trait Strategy {
    fn solve(&self, grid: &Grid) -> Grid;
    fn name(&self) -> String;
}

use trivial::PruneCandidates;

use easy::HiddenSingle;

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(PruneCandidates), Box::new(HiddenSingle)]
}
