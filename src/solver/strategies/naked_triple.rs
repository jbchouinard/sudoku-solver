use super::{Difficulty, StrategyDelta, UnitStrategy};
use crate::{Cell, Grid, Unit};

#[derive(Clone)]
pub struct NakedTriple;

impl UnitStrategy for NakedTriple {
    fn name(&self) -> String {
        "Naked Triple".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }

    fn solve_unit(&self, _grid: &Grid, unit: &Unit) -> StrategyDelta {
        let mut delta = StrategyDelta::new();
        let mut unsolved = vec![];
        for (p, cell) in unit {
            if let Cell::Unsolved(candidates) = cell {
                unsolved.push((p, candidates));
            }
        }
        // TODO: Surely there is a smarter way to do this...
        for i in 0..unsolved.len() {
            let (p1, c1) = &unsolved[i];
            if c1.count() <= 3 {
                for j in i + 1..unsolved.len() {
                    let (p2, c2) = &unsolved[j];
                    let c1_c2 = c1.combine(c2);
                    if c1_c2.count() <= 3 {
                        for k in j + 1..unsolved.len() {
                            let (p3, c3) = &unsolved[k];
                            let c1_c2_c3 = c1_c2.combine(c3);
                            if c1_c2_c3.count() == 3 {
                                for (pu, cu) in &unsolved {
                                    if pu != p1 && pu != p2 && pu != p3 {
                                        for cval in c1_c2_c3.to_vec() {
                                            if cu.can_be(&cval) {
                                                delta.eliminate(**pu, cval)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        delta
    }
}
