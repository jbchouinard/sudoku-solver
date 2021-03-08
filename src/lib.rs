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
        if !(1..=9).contains(&val) {
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

impl From<CellValue> for u8 {
    fn from(val: CellValue) -> u8 {
        val.0
    }
}

impl From<CellValue> for usize {
    fn from(val: CellValue) -> usize {
        let n: u8 = val.into();
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
        for (i, b) in self.0.iter().enumerate() {
            if *b {
                v.push(CellValue::new((i + 1).try_into().unwrap()));
            }
        }
        v
    }

    pub fn count(&self) -> u8 {
        let mut c: u8 = 0;
        for b in self.0.iter() {
            if *b {
                c += 1;
            }
        }
        c
    }

    pub fn combine(&self, other: &Candidates) -> Candidates {
        let mut possible = [false; 9];
        for (i, p) in possible.iter_mut().enumerate() {
            if self.0[i] || other.0[i] {
                *p = true;
            }
        }
        Candidates(possible)
    }

    fn index(v: &CellValue) -> usize {
        let n: u8 = (*v).into();
        (n - 1).into()
    }

    pub fn add(&mut self, v: &CellValue) {
        self.0[Self::index(v)] = true;
    }

    pub fn remove(&mut self, v: &CellValue) {
        self.0[Self::index(v)] = false;
    }

    pub fn can_be(&self, v: &CellValue) -> bool {
        self.0[Self::index(v)]
    }
}

impl fmt::Display for Candidates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let mut parts = vec![];
        for v in self.to_vec() {
            let n: u8 = v.into();
            parts.push(n.to_string());
        }
        write!(f, "[{}]", parts.join(","))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Solved(CellValue),
    Unsolved(Candidates),
}

impl Cell {
    pub fn from_u8(val: u8) -> Cell {
        if (1..=9).contains(&val) {
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n: u8 = match self {
            Self::Solved(v) => (*v).into(),
            Self::Unsolved(_) => 0,
        };
        write!(f, "{}", n)
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
        if !(1..=9).contains(&col) || (row < 1) || (row > 9) {
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

    /// All positions in a grid
    pub fn grid_vec() -> Vec<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            for row in 1..=9 {
                v.push(Position::new(col, row));
            }
        }
        v
    }

    /// Positions for the row containing this position
    pub fn row_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = vec![];
        for col in 1..=9 {
            if include_self || col != self.col {
                v.push(Position::new(col, self.row));
            }
        }
        v
    }

    /// Positions for the column containing this position
    pub fn col_vec(&self, include_self: bool) -> Vec<Self> {
        let mut v = vec![];
        for row in 1..=9 {
            if include_self || row != self.row {
                v.push(Position::new(self.col, row));
            }
        }
        v
    }

    /// Positions for the box containing this position
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

    /// Positions for all units (row, column, box) in a grid
    pub fn unit_vecs() -> Vec<Vec<Self>> {
        let mut units = vec![];
        // Iterate each unit (row, col, box)
        for r in 1..=9 {
            units.push(Position::new(1, r).row_vec(true));
        }
        for c in 1..=9 {
            units.push(Position::new(c, 1).col_vec(true));
        }
        for r in (1..=9).step_by(3) {
            for c in (1..=9).step_by(3) {
                units.push(Position::new(c, r).box_vec(true));
            }
        }
        units
    }

    /// Positions "seen" by this posititon (in the same row, column or box)
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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "R{}C{}", self.row, self.col)
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

    pub fn set_cell(&mut self, pos: Position, cell: Cell) -> bool {
        let new_cell = cell.to_solved();
        let old_cell = self.get_cell(pos);
        if new_cell == old_cell {
            false
        } else {
            self.cells[pos.index()] = new_cell;
            true
        }
    }

    pub fn get_cells(&self, pos: Vec<Position>) -> Unit {
        let mut u = Unit::new();
        for p in pos {
            u.insert(p, self.get_cell(p));
        }
        u
    }

    pub fn set_cells(&mut self, unit: Unit) -> u16 {
        let mut changed_count = 0;
        for (p, cell) in unit.into_iter() {
            if self.set_cell(p, cell) {
                changed_count += 1;
            }
        }
        changed_count
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..81 {
            if let Cell::Unsolved(_) = self.cells[i] {
                return false;
            }
        }
        true
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = vec![];
        for v in self.cells.iter() {
            parts.push(v.to_string());
        }
        write!(f, "{}", parts.join(""))
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
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
