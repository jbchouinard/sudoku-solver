#[macro_use]
extern crate lazy_static;

use std::convert::TryInto;
use std::fmt;

pub mod render;

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

    pub fn from_string(cell_values: &str) -> Result<SudokuGrid> {
        if cell_values.len() != 81 {
            Err(Error::new("Puzzle string must have 81 digits"))
        } else {
            let char_vec: Vec<char> = cell_values.chars().collect();
            let char_arr: [char; 81] = (char_vec[0..81]).try_into().unwrap();
            Ok(SudokuGrid::from_chars(&char_arr))
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

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
