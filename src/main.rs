use quicli::prelude::*;
use structopt::StructOpt;
use sudoku::html::SolverRenderer;
use sudoku::solve::{strategies::all_strategies, Solver};
use sudoku::Grid;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "outdir", short = "o", default_value = "html")]
    outdir: String,
    puzzle: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let sudoku = Grid::from_string(&args.puzzle)?;
    let solver = SolverRenderer::new(Solver::new(all_strategies()));
    solver.solve_and_render(&sudoku, &args.outdir)?;
    Ok(())
}
