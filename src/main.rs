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
    let sudoku = Grid::from_str(puzzle).unwrap();
    let solver = Solver::new(all_strategies());
    let start = Instant::now();
    let solution = solver.solve(&sudoku);
    let mut step = 0;
    let solved_grid;
    for solution_step in solution.steps {
        eprintln!(
            "{} {}: solved {} cells, eliminated {} candidates ({} μs)",
            step,
            &solution_step.strategy.name(),
            solution_step.solved_diff.cells_solved,
            solution_step.solved_diff.candidates_eliminated,
            solution_step.time.as_micros(),
        );
        step += 1;
    }
    eprintln!("Total time: {} ms", start.elapsed().as_millis());
    solved_grid = solution.grid;
    if solved_grid.is_solved() {
        eprintln!("Solved!");
    } else {
        eprintln!("Uh-oh, we couldn't solve that one.");
    }
    println!("{}", solved_grid.to_string());
}

#[cfg(feature = "html")]
fn solve_and_render_html(puzzle: &str, out_dir: &str) {
    let sudoku = Grid::from_str(puzzle).unwrap();
    let solver = Solver::new(all_strategies());
    let renderer = SolverRenderer::new(solver);
    let solved_grid = renderer.solve_and_render(&sudoku, out_dir).unwrap();
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
