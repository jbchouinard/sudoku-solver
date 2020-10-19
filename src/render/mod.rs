use crate::{SudokuCell, SudokuGrid};
use std::include_str;
use tera::{Context, Tera};

pub mod explainer;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("header.html", include_str!("templates/header.html"))
            .unwrap();
        tera.add_raw_template("footer.html", include_str!("templates/footer.html"))
            .unwrap();
        tera.add_raw_template("grid.html", include_str!("templates/grid.html"))
            .unwrap();
        tera.add_raw_template("mark.html", include_str!("templates/mark.html"))
            .unwrap();
        tera.add_raw_template("number.html", include_str!("templates/number.html"))
            .unwrap();
        tera.add_raw_template("sudoku.html", include_str!("templates/sudoku.html"))
            .unwrap();
        tera.add_raw_template(
            "sudoku_step.html",
            include_str!("templates/sudoku_step.html"),
        )
        .unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

impl SudokuGrid {
    pub fn tera_context(&self) -> Context {
        let mut cells: Vec<String> = Vec::new();
        for cell in self.grid.iter() {
            cells.push(cell.render_html());
        }
        let mut context = Context::new();
        context.insert("cells", &cells[0..81]);
        context
    }

    pub fn render_html(&self) -> String {
        TERA.render("sudoku.html", &self.tera_context()).unwrap()
    }
}

impl SudokuCell {
    pub fn render_html(&self) -> String {
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
