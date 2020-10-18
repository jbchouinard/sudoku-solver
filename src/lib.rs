#[macro_use]
extern crate lazy_static;

use std::convert::TryInto;

pub mod render;

pub struct Error;

#[derive(Copy, Clone)]
pub enum SudokuCell {
    Solved(u8),
    Unsolved([bool; 9]),
}

impl SudokuCell {
    pub fn new(val: u8) -> SudokuCell {
        if (val >= 1) && (val <= 9) {
            SudokuCell::Solved(val)
        } else {
            SudokuCell::Unsolved([true; 9])
        }
    }
}

#[derive(Copy, Clone)]
pub struct SudokuGrid {
    grid: [SudokuCell; 81],
}

impl SudokuGrid {
    pub fn new() -> SudokuGrid {
        SudokuGrid {
            grid: [SudokuCell::new(0); 81],
        }
    }

    pub fn from_chars(cell_values: &[char; 81]) -> SudokuGrid {
        let mut cells: Vec<SudokuCell> = Vec::new();
        for c in cell_values.iter() {
            let n: u8 = match c.to_digit(10) {
                Some(n) => {
                    if (n >= 1) && (n <= 9) {
                        n.try_into().unwrap()
                    } else {
                        0
                    }
                }
                None => 0,
            };
            cells.push(SudokuCell::new(n));
        }
        SudokuGrid {
            grid: cells[0..81].try_into().unwrap(),
        }
    }
}
