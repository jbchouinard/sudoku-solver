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

fn solve_and_print(puzzle: &str) {
    let mut sudoku = Grid::from_str(puzzle).unwrap();
    let solver = Solver::new(all_strategies());
    let start = Instant::now();
    let steps = solver.solve(&mut sudoku);
    for (i, step) in steps.iter().enumerate() {
        eprintln!(
            "{} {}: {} ({} μs)",
            i,
            &step.strategy.name(),
            step.delta,
            step.time.as_micros(),
        );
    }
    eprintln!("Total time: {} ms", start.elapsed().as_millis());
    if sudoku.is_solved() {
        eprintln!("Solved!");
    } else {
        eprintln!("Uh-oh, we couldn't solve that one.");
    }
    println!("{}", sudoku.to_string());
}

#[cfg(feature = "html")]
fn solve_and_render_html(puzzle: &str, out_dir: &str) {
    let sudoku = Grid::from_str(puzzle).unwrap();
    let solver = Solver::new(all_strategies());
    let renderer = SolverRenderer::new(solver);
    let solved_grid = renderer.solve_and_render(sudoku, out_dir).unwrap();
    println!("{}", solved_grid.to_string());
}

fn main() {
    let args = Cli::from_args();

    #[cfg(feature = "html")]
    match args.html_output_dir {
        Some(dir) => {
            solve_and_render_html(&args.puzzle, &dir);
        }
        None => {
            solve_and_print(&args.puzzle);
        }
    }

    #[cfg(not(feature = "html"))]
    solve_and_print(&args.puzzle);
}
