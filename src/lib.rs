use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[cfg(feature = "html")]
pub mod html;

pub mod solver;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CellValue(u8);

impl CellValue {
    pub fn new(val: u8) -> Self {
        if (val < 1) || (val > 9) {
            panic!("invalid CellValue");
        }
        Self(val)
    }
}

impl From<u8> for CellValue {
    fn from(val: u8) -> Self {
        Self::new(val)
    }
}

impl Into<u8> for CellValue {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<usize> for CellValue {
    fn into(self) -> usize {
        let n: u8 = self.into();
        n.into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Candidates([bool; 9]);

impl Candidates {
    pub fn new(possible: [bool; 9]) -> Self {
        Self(possible)
    }

    pub fn to_vec(&self) -> Vec<CellValue> {
        let mut v = vec![];
        for i in 0..9 {
            if self.0[i] {
                v.push(CellValue::new((i + 1).try_into().unwrap()));
            }
        }
        v
    }

    pub fn count(&self) -> u8 {
        let mut c: u8 = 0;
        for i in 0..9 {
            if self.0[i] {
                c = c + 1;
            }
        }
        c
    }

    pub fn combine(&self, other: &Candidates) -> Candidates {
        let mut possible = [false; 9];
        for i in 0..9 {
            if self.0[i] || other.0[i] {
                possible[i] = true;
            }
        }
        Candidates(possible)
    }

    fn index(v: &CellValue) -> usize {
        let n: u8 = v.clone().into();
        (n - 1).into()
    }

    pub fn add(&mut self, v: &CellValue) {
        self.0[Self::index(v)] = false;
    }

    pub fn remove(&mut self, v: &CellValue) {
        self.0[Self::index(v)] = false;
    }

    pub fn can_be(&self, v: &CellValue) -> bool {
        self.0[Self::index(v)]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Solved(CellValue),
    Unsolved(Candidates),
}

impl Cell {
    pub fn from_u8(val: u8) -> Cell {
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
        self
    }

    pub fn candidates(&self) -> Option<Vec<CellValue>> {
        match self {
            Self::Solved(_) => None,
            Self::Unsolved(candidates) => Some(candidates.to_vec()),
        }
    }

    pub fn can_be(&self, v: &CellValue) -> bool {
        match self {
            Self::Solved(ov) => v == ov,
            Self::Unsolved(c) => c.can_be(v),
        }
    }
}

impl ToString for Cell {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::Solved(v) => {
                let n: u8 = v.clone().into();
                n.to_string()
            }
            Self::Unsolved(_) => "0".to_string(),
        }
    }
}

impl FromStr for Cell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let n: u8 = s.parse::<u8>()?;
        Ok(Cell::from_u8(n))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

    pub fn grid_vec() -> Vec<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            for row in 1..=9 {
                v.push(Position::new(col, row));
            }
        }
        v
    }

    pub fn row_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            if include_self || col != self.col {
                v.push(Position::new(col, self.row));
            }
        }
        v
    }

    pub fn col_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = vec![];
        for row in 1..=9 {
            if include_self || row != self.row {
                v.push(Position::new(self.col, row));
            }
        }
        v
    }

    pub fn box_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = vec![];
        for col in Self::box_range(self.col) {
            for row in Self::box_range(self.row) {
                if include_self || row != self.row || col != self.col {
                    v.push(Position::new(col, row))
                }
            }
        }
        v
    }

    pub fn seen_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = self.box_vec(include_self);
        for p in self.col_vec(include_self).into_iter() {
            if !v.contains(&p) {
                v.push(p)
            }
        }
        for p in self.row_vec(include_self).into_iter() {
            if !v.contains(&p) {
                v.push(p)
            }
        }
        v
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Grid {
    cells: [Cell; 81],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: [Cell::from_u8(0); 81],
        }
    }

    pub fn get_cell(&self, pos: Position) -> Cell {
        self.cells[pos.index()]
    }

    pub fn set_cell(&mut self, pos: Position, cell: Cell) {
        self.cells[pos.index()] = cell.to_solved();
    }

    pub fn get_cells(&self, pos: Vec<Position>) -> Unit {
        let mut u = Unit::new();
        for p in pos {
            u.insert(p, self.get_cell(p));
        }
        u
    }

    pub fn set_cells(&mut self, unit: Unit) {
        for (p, cell) in unit.into_iter() {
            self.set_cell(p, cell);
        }
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..81 {
            if let Cell::Unsolved(_) = self.cells[i] {
                return false;
            }
        }
        true
    }

    pub fn solved_diff_from(&self, solved: &Grid) -> GridSolvedDiff {
        let mut cells_solved = 0;
        let mut candidates_eliminated = 0;
        for i in 0..81 {
            let cell = self.cells[i];
            let solcell = solved.cells[i];
            if let Cell::Unsolved(candx) = cell {
                match solcell {
                    Cell::Solved(_) => {
                        cells_solved += 1;
                    }
                    Cell::Unsolved(solved_candx) => {
                        candidates_eliminated += candx.to_vec().len() - solved_candx.to_vec().len()
                    }
                }
            }
        }
        GridSolvedDiff {
            cells_solved,
            candidates_eliminated: candidates_eliminated.try_into().unwrap(),
        }
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Grid> {
        if s.len() != 81 {
            return Err(Error::new("Puzzle string must have 81 characters"));
        }
        let mut cells = vec![];
        for i in 0..81 {
            cells.push(Cell::from_str(&s[i..i + 1]).unwrap())
        }
        Ok(Grid {
            cells: cells[0..81].try_into().unwrap(),
        })
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut parts = vec![];
        for v in self.cells.iter() {
            parts.push(v.to_string());
        }
        parts.join("")
    }
}

pub struct GridSolvedDiff {
    pub cells_solved: u16,
    pub candidates_eliminated: u16,
}

type Unit = HashMap<Position, Cell>;

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::new("Error parsing int")
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
