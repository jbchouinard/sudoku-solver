use quicli::prelude::*;
use std::fs;

use structopt::StructOpt;

use sudoku::solve::*;
use sudoku::SudokuGrid;

#[derive(Debug, StructOpt)]
struct Cli {
    file_out: String,
    puzzle: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let sudoku = SudokuGrid::from_string(&args.puzzle)?;
    let solver = SudokuSolver::new(all_strategies());
    let (strats, solved) = solver.solve(&sudoku);
    fs::write(args.file_out, solved.to_html())?;
    for strat in strats {
        println!("{}", strat.name());
    }
    Ok(())
}
