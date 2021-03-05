use crate::solver::strategies::Strategy;
use crate::{Cell, CellValue, Grid, Position};

pub struct HiddenSingle;

impl HiddenSingle {
    fn is_hidden_single(grid: &Grid, p: &Position, v: &CellValue) -> bool {
        let mut col_has_same = false;
        for same_col in p.iter_col(false) {
            if grid.get_cell(same_col).can_be(v) {
                col_has_same = true;
                break;
            }
        }
        if !col_has_same {
            return true;
        }
        let mut row_has_same = false;
        for same_row in p.iter_row(false) {
            if grid.get_cell(same_row).can_be(v) {
                row_has_same = true;
                break;
            }
        }
        if !row_has_same {
            return true;
        }
        let mut box_has_same = false;
        for same_box in p.iter_box(false) {
            if grid.get_cell(same_box).can_be(v) {
                box_has_same = true;
                break;
            }
        }
        if !box_has_same {
            return true;
        }
        false
    }

    fn solve_cell(grid: &Grid, p: Position) -> Cell {
        let cell = grid.get_cell(p);
        match cell {
            Cell::Solved(_) => cell.clone(),
            Cell::Unsolved(candidates) => {
                for candidate_value in candidates.to_vec().into_iter() {
                    if Self::is_hidden_single(grid, &p, &candidate_value) {
                        return Cell::Solved(candidate_value);
                    }
                }
                cell.clone()
            }
        }
    }
}

impl Strategy for HiddenSingle {
    fn solve(&self, grid: &Grid) -> Grid {
        let mut new_grid = grid.clone();
        for p in Position::iter_grid() {
            new_grid.set_cell(p, Self::solve_cell(grid, p));
        }
        new_grid
    }
    fn name(&self) -> String {
        "[Easy] Hidden Single".to_string()
    }
}
