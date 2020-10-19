use quicli::prelude::*;
use structopt::StructOpt;
use sudoku::render::explainer::SolutionExplainer;
use sudoku::solve::*;
use sudoku::SudokuGrid;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "outdir", short = "o", default_value = "html")]
    outdir: String,
    puzzle: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let sudoku = SudokuGrid::from_string(&args.puzzle)?;
    let solver = SudokuSolver::new(all_strategies());
    let mut explainer = SolutionExplainer::new(solver);
    explainer.solve(&sudoku);
    explainer.render(&args.outdir)?;
    Ok(())
}
