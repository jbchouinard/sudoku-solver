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

pub struct SudokuGrid {
    grid: [SudokuCell; 81],
}

impl SudokuGrid {
    pub fn new() -> SudokuGrid {
        SudokuGrid {
            grid: [SudokuCell::new(0); 81],
        }
    }

    pub fn from_string(cell_values: &[char; 81]) -> SudokuGrid {
        let mut cells: Vec<SudokuCell> = Vec::new();

        SudokuGrid::new()
    }
}

pub fn do_thing() {}
