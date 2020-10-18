use quicli::prelude::*;
use structopt::StructOpt;

use sudoku::SudokuGrid;

#[derive(Debug, StructOpt)]
struct Cli {
    puzzle: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let grid = SudokuGrid::from_string(&args.puzzle)?;
    println!("{}", grid.to_html());
    Ok(())
}
