use crate::render::TERA;
use crate::solve::SudokuSolver;
use crate::SudokuGrid;

use std::fs;

pub struct SolutionExplainer {
    solver: SudokuSolver,
    strategies: Vec<String>,
    grids: Vec<SudokuGrid>,
}

impl SolutionExplainer {
    pub fn new(solver: SudokuSolver) -> Self {
        Self {
            solver: solver,
            strategies: vec![],
            grids: vec![],
        }
    }

    pub fn solve(&mut self, sudoku: &SudokuGrid) {
        self.grids.push(sudoku.clone());
        let mut solution = sudoku.clone();
        loop {
            match self.solver.solve_step(&solution) {
                Some((strat, sol)) => {
                    self.strategies.push(strat.name());
                    self.grids.push(sol.clone());
                    solution = sol;
                }
                None => {
                    break;
                }
            }
        }
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

    pub fn render(&self, output_dir: &str) -> Result<(), std::io::Error> {
        let count = self.grids.len();
        for i in 0..count {
            let mut strat = "Start";
            let mut link_prev = "".to_string();
            let mut link_next = "".to_string();
            if i > 0 {
                strat = &self.strategies[i - 1];
                link_prev = Self::link(i - 1)
            }
            if i < count - 1 {
                link_next = Self::link(i + 1)
            }
            let grid = self.grids[i];
            let mut context = grid.tera_context();
            context.insert("link_next", &link_next);
            context.insert("link_prev", &link_prev);
            context.insert("strategy", strat);
            let html = TERA.render("sudoku_step.html", &context).unwrap();
            let fileout = format!("{}/{}", output_dir, Self::link(i));
            fs::write(fileout, html)?;
        }
        Ok(())
    }
}
