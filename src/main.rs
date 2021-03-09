use std::str::FromStr;
use std::time::Instant;

use structopt::StructOpt;

#[cfg(feature = "html")]
use sudoku::html::SolverRenderer;

use sudoku::solver::{strategies::all_strategies, Solver};
use sudoku::Grid;

#[derive(Debug, StructOpt)]
struct Cli {
    #[cfg(feature = "html")]
    #[structopt(long = "output-html", short = "o")]
    html_output_dir: Option<String>,
    puzzle: String,
}

fn solve_and_print(solver: Solver, mut sudoku: Grid) {
    let start = Instant::now();
    let steps = solver.solve(&mut sudoku);
    for (i, step) in steps.iter().enumerate() {
        eprintln!(
            "{} {} ({} μs) : {} ",
            i,
            &step.strategy.name(),
            step.time.as_micros(),
            step.delta,
        );
    }
    eprintln!("Total time: {} ms", start.elapsed().as_millis());
    println!("{}", sudoku);
}

#[cfg(feature = "html")]
fn solve_and_render_html(solver: Solver, mut sudoku: Grid, out_dir: &str) {
    let renderer = SolverRenderer::new(solver);
    renderer.solve_and_render(&mut sudoku, out_dir).unwrap();
    println!("{}", sudoku.to_string());
}

fn main() {
    let args = Cli::from_args();
    let sudoku = Grid::from_str(&args.puzzle).unwrap();
    let solver = Solver::new(all_strategies());

    #[cfg(feature = "html")]
    match args.html_output_dir {
        Some(dir) => {
            solve_and_render_html(solver, sudoku, &dir);
        }
        None => {
            solve_and_print(solver, sudoku);
        }
    }

    #[cfg(not(feature = "html"))]
    solve_and_print(&args.puzzle);
}
