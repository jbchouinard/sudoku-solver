use super::{AnyStrategy, Difficulty, UnitStrategy};
use crate::{Cell, Grid, Unit};

pub struct NakedTriple;

impl AnyStrategy for NakedTriple {
    fn name(&self) -> String {
        "Naked Triple".to_string()
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Standard
    }
}

impl UnitStrategy for NakedTriple {
    fn solve_unit(&self, _grid: &Grid, unit: &Unit) -> Unit {
        let mut solved_unit = Unit::new();
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
                                        let mut pruned_candidates = (*cu).clone();
                                        for cval in c1_c2_c3.to_vec() {
                                            pruned_candidates.remove(&cval);
                                        }
                                        solved_unit.insert(**pu, Cell::Unsolved(pruned_candidates));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        solved_unit
    }
}
