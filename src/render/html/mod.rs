use crate::{SudokuCell, SudokuGrid};
use std::include_str;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        let mark_tmpl = include_str!("templates/mark.html");
        tera.add_raw_template("mark.html", mark_tmpl).unwrap();
        let number_tmpl = include_str!("templates/number.html");
        tera.add_raw_template("number.html", number_tmpl).unwrap();
        let sudoku_tmpl = include_str!("templates/sudoku.html");
        tera.add_raw_template("sudoku.html", sudoku_tmpl).unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

impl SudokuGrid {
    pub fn to_html(&self) -> String {
        let mut cells: Vec<String> = Vec::new();
        for cell in self.grid.iter() {
            cells.push(cell.to_html());
        }
        let mut context = Context::new();
        context.insert("cells", &cells[0..81]);
        TERA.render("sudoku.html", &context).unwrap()
    }
}

impl SudokuCell {
    pub fn to_html(&self) -> String {
        let mut context = Context::new();
        match self {
            SudokuCell::Solved(cell) => {
                context.insert("cell", &cell);
                TERA.render("number.html", &context).unwrap()
            }
            SudokuCell::Unsolved(mcells) => {
                context.insert("mcells", &mcells);
                TERA.render("mark.html", &context).unwrap()
            }
        }
    }
}
