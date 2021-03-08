use std::fs;
use std::include_str;

use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::solver::strategies::Strategy;
use crate::solver::Solver;
use crate::{Cell, Grid};

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

pub struct GridRenderer<'a> {
    grid: &'a Grid,
}

impl<'a> GridRenderer<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    pub fn tera_context(&self) -> Context {
        let mut cells: Vec<String> = Vec::new();
        for cell in self.grid.cells.iter() {
            cells.push(CellRenderer::new(cell).render());
        }
        let mut context = Context::new();
        context.insert("cells", &cells[0..81]);
        context
    }

    pub fn render(&self) -> String {
        TERA.render("sudoku.html", &self.tera_context()).unwrap()
    }
}

pub struct CellRenderer<'a> {
    cell: &'a Cell,
}

impl<'a> CellRenderer<'a> {
    pub fn new(cell: &'a Cell) -> Self {
        Self { cell }
    }

    pub fn render(&self) -> String {
        let mut context = Context::new();
        match self.cell {
            Cell::Solved(cell) => {
                let v: u8 = cell.clone().into();
                context.insert("cell", &v);
                TERA.render("number.html", &context).unwrap()
            }
            Cell::Unsolved(mcells) => {
                context.insert("mcells", &mcells.0);
                TERA.render("mark.html", &context).unwrap()
            }
        }
    }
}

pub struct SolverRenderer {
    solver: Solver,
}

impl SolverRenderer {
    pub fn new(solver: Solver) -> Self {
        Self { solver }
    }

    fn link(n: usize) -> String {
        let name: String;
        if n == 0 {
            name = "index".to_string();
        } else {
            name = format!("step{}", n);
        }
        format!("{}.html", name)
    }

    fn strategy_string(strat: Option<&Strategy>) -> String {
        match strat {
            Some(strategy) => {
                format!(
                    "[{}] {}",
                    strategy.difficulty(),
                    strategy.name()
                )
            }
            None => "Start".to_string(),
        }
    }

    fn render_step(
        &self,
        grid: &Grid,
        step: usize,
        strat: &str,
        link_prev: bool,
        link_next: bool,
    ) -> String {
        let mut context = GridRenderer::new(grid).tera_context();
        let link_prev_url = if link_prev {
            Self::link(step - 1)
        } else {
            "".to_string()
        };
        let link_next_url = if link_next {
            Self::link(step + 1)
        } else {
            "".to_string()
        };
        context.insert("strategy", strat);
        context.insert("link_prev", &link_prev_url);
        context.insert("link_next", &link_next_url);
        TERA.render("sudoku_step.html", &context).unwrap()
    }

    pub fn solve_and_render(
        &self,
        sudoku: &Grid,
        output_dir: &str,
    ) -> Result<Grid, std::io::Error> {
        let mut step = 0;
        let mut current_strat;
        let mut next_strat = None;
        let mut prev_grid;
        let mut current_grid = None;
        let mut next_grid = Some(*sudoku);
        loop {
            if next_grid.is_none() {
                break;
            }

            prev_grid = current_grid;
            current_grid = next_grid;
            current_strat = next_strat;
            step += 1;

            match self.solver.solve_step(&current_grid.unwrap()) {
                Some((strategy, solution, _)) => {
                    next_grid = Some(solution);
                    next_strat = Some(strategy);
                }
                None => {
                    next_grid = None;
                }
            }
            fs::write(
                format!("{}/{}", output_dir, Self::link(step)),
                self.render_step(
                    &current_grid.unwrap(),
                    step,
                    &Self::strategy_string(current_strat),
                    prev_grid.is_some(),
                    next_grid.is_some(),
                ),
            )?;
        }
        Ok(current_grid.unwrap())
    }
}
