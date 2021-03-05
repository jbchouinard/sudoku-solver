use quicli::prelude::*;
use structopt::StructOpt;
use sudoku::html::SolverRenderer;
use sudoku::solver::{strategies::all_strategies, Solver};
use sudoku::Grid;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "output-html", short = "o")]
    html_output_dir: Option<String>,
    puzzle: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let sudoku = Grid::from_string(&args.puzzle)?;
    let solver = Solver::new(all_strategies());

    match args.html_output_dir {
        Some(dir) => {
            let renderer = SolverRenderer::new(solver);
            renderer.solve_and_render(&sudoku, &dir)?;
        }
        None => {
            let solution = solver.solve(&sudoku);
            println!("{}", solution.to_string());
        }
    }
    Ok(())
}
