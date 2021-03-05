#[macro_use]
extern crate lazy_static;

use std::convert::TryInto;
use std::fmt;
use std::vec::IntoIter;

pub mod html;
pub mod solve;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CellValue {
    value: u8,
}

impl CellValue {
    pub fn new(val: u8) -> Self {
        if (val < 1) || (val > 9) {
            panic!("invalid CellValue");
        }
        Self { value: val }
    }
}

impl From<u8> for CellValue {
    fn from(val: u8) -> Self {
        Self::new(val)
    }
}

impl Into<u8> for CellValue {
    fn into(self) -> u8 {
        self.value
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Candidates {
    possible: [bool; 9],
}

impl Candidates {
    pub fn new(possible: [bool; 9]) -> Self {
        Self { possible }
    }

    pub fn to_vec(&self) -> Vec<CellValue> {
        let mut v = vec![];
        for i in 0..9 {
            if self.possible[i] {
                v.push(CellValue::new((i + 1).try_into().unwrap()));
            }
        }
        v
    }

    fn index(v: &CellValue) -> usize {
        let n: u8 = v.clone().into();
        (n - 1).into()
    }

    pub fn add(&mut self, v: &CellValue) {
        self.possible[Self::index(v)] = false;
    }

    pub fn remove(&mut self, v: &CellValue) {
        self.possible[Self::index(v)] = false;
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Solved(CellValue),
    Unsolved(Candidates),
}

impl Cell {
    pub fn new(val: u8) -> Cell {
        if (val >= 1) && (val <= 9) {
            Self::Solved(val.into())
        } else {
            Self::Unsolved(Candidates::new([true; 9]))
        }
    }

    pub fn to_solved(self) -> Self {
        if let Some(candidates) = self.candidates() {
            if candidates.len() == 1 {
                return Cell::Solved(candidates[0]);
            }
        }
        self.clone()
    }

    pub fn candidates(&self) -> Option<Vec<CellValue>> {
        match self {
            Self::Solved(_) => None,
            Self::Unsolved(candidates) => Some(candidates.to_vec()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    col: u8,
    row: u8,
}

impl Position {
    pub fn new(col: u8, row: u8) -> Position {
        if (col < 1) || (col > 9) || (row < 1) || (row > 9) {
            panic!("out of bounds");
        }
        Self { col, row }
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn index(&self) -> usize {
        ((self.col - 1) + (self.row - 1) * 9).into()
    }

    fn box_range(coord: u8) -> std::ops::Range<u8> {
        let low = 1 + ((coord - 1) / 3) * 3;
        low..low + 3
    }

    pub fn iter_grid() -> IntoIter<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            for row in 1..=9 {
                v.push(Position::new(col, row));
            }
        }
        v.into_iter()
    }

    pub fn iter_row(&self) -> IntoIter<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            v.push(Position::new(col, self.row));
        }
        v.into_iter()
    }

    pub fn iter_col(&self) -> IntoIter<Self> {
        let mut v = vec![];
        for row in 1..=9 {
            v.push(Position::new(self.col, row));
        }
        v.into_iter()
    }

    pub fn iter_box(&self) -> IntoIter<Self> {
        let mut v = vec![];
        for col in Self::box_range(self.col) {
            for row in Self::box_range(self.row) {
                v.push(Position::new(col, row))
            }
        }
        v.into_iter()
    }

    pub fn iter_seen(&self) -> IntoIter<Self> {
        // TODO: don't iterate over same cells more than once
        self.iter_row()
            .chain(self.iter_col())
            .chain(self.iter_box())
            .collect::<Vec<Self>>()
            .into_iter()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Grid {
    grid: [Cell; 81],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: [Cell::new(0); 81],
        }
    }

    pub fn get_cell(&self, pos: Position) -> &Cell {
        &self.grid[pos.index()]
    }

    pub fn set_cell(&mut self, pos: Position, cell: Cell) {
        self.grid[pos.index()] = cell.to_solved();
    }

    pub fn from_string(cell_values: &str) -> Result<Grid> {
        if cell_values.len() != 81 {
            Err(Error::new("Puzzle string must have 81 digits"))
        } else {
            let char_vec: Vec<char> = cell_values.chars().collect();
            let char_arr: [char; 81] = (char_vec[0..81]).try_into().unwrap();
            Ok(Grid::from_chars(&char_arr))
        }
    }

    pub fn from_chars(cell_values: &[char; 81]) -> Grid {
        let mut cells: Vec<Cell> = Vec::new();
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
            cells.push(Cell::new(n));
        }
        Grid {
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
