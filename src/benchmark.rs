use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

use num_cpus;
use structopt::StructOpt;

use sudoku::solver::strategies::all_strategies;
use sudoku::solver::{SolutionStep, Solver};
use sudoku::stats::{Count, Formatted, Maximum, Mean, Minimum, Report, ReportBuilder};
use sudoku::{Error, Grid};

const PUZZLE_STR: &str = include_str!("../puzzles/benchmark.txt");

struct Puzzle {
    grid: Grid,
    rating: f64,
}

impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        let [_, puzzle_str, rating]: [&str; 3] = s
            .split(' ')
            .collect::<Vec<&str>>()
            .try_into()
            .expect("puzzle should be str like '<id> <puzzle> <rating>'");
        Ok(Puzzle {
            grid: Grid::from_str(puzzle_str)?,
            rating: rating.parse().unwrap(),
        })
    }
}

fn parse_puzzles(s: &str) -> Vec<Puzzle> {
    s.trim()
        .split('\n')
        .map(|s| Puzzle::from_str(s).unwrap())
        .collect()
}

struct BenchmarkReport {
    report: Report<f64>,
}

impl BenchmarkReport {
    fn new() -> Self {
        BenchmarkReport {
            report: ReportBuilder::new()
                .with("Puzzles", Formatted::new(Box::new(Count::new()), "", 0))
                .with("% Solved", Formatted::new(Box::new(Mean::new()), "%", 1))
                .with(
                    "Solve Steps",
                    Formatted::new(Box::new(Minimum::new()), "", 0),
                )
                .with("Solve Steps", Formatted::new(Box::new(Mean::new()), "", 0))
                .with(
                    "Solve Steps",
                    Formatted::new(Box::new(Maximum::new()), "", 0),
                )
                .with(
                    "Difficulty Rating",
                    Formatted::new(Box::new(Minimum::new()), "", 1),
                )
                .with(
                    "Difficulty Rating",
                    Formatted::new(Box::new(Mean::new()), "", 1),
                )
                .with(
                    "Difficulty Rating",
                    Formatted::new(Box::new(Maximum::new()), "", 1),
                )
                .with(
                    "Solve Time",
                    Formatted::new(Box::new(Minimum::new()), "ms", 0),
                )
                .with("Solve Time", Formatted::new(Box::new(Mean::new()), "ms", 0))
                .with(
                    "Solve Time",
                    Formatted::new(Box::new(Maximum::new()), "ms", 1),
                )
                .build(),
        }
    }

    fn add_measurement(&mut self, m: &Measurement) {
        self.report.update("Puzzles", 1.0);
        self.report.update("Difficulty Rating", m.rating);
        self.report.update("Solve Steps", m.solve_steps);
        self.report.update("Solve Time", m.solve_time);
        if m.solved {
            self.report.update("% Solved", 100.0);
        } else {
            self.report.update("% Solved", 0.0);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Measurement {
    solved: bool,
    rating: f64,
    solve_time: f64,
    solve_steps: f64,
}

impl Measurement {
    fn new(puzzle: &Puzzle, steps: &[SolutionStep]) -> Self {
        Measurement {
            solved: puzzle.grid.is_solved(),
            rating: puzzle.rating,
            solve_time: Self::total_time(steps),
            solve_steps: steps.len() as f64,
        }
    }
    fn total_time(steps: &[SolutionStep]) -> f64 {
        steps
            .iter()
            .map(|s| (s.time.as_micros() as f64) / 1000.0)
            .sum()
    }
}

impl fmt::Display for BenchmarkReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.report)
    }
}

fn run_benchmark(puzzles: Vec<Puzzle>) -> Vec<Measurement> {
    let mut ms = vec![];
    let solver = Solver::new(all_strategies());
    for mut puz in puzzles {
        let sol = solver.solve(&mut puz.grid);
        ms.push(Measurement::new(&puz, &sol));
    }
    ms
}

fn round_robin_split<T>(vec: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut vv: Vec<Vec<T>> = (0..n).map(|_| vec![]).collect();
    for (i, elem) in vec.into_iter().enumerate() {
        vv[i % n].push(elem);
    }
    vv
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long, short)]
    threads: Option<usize>,
}

fn main() {
    let args = Cli::from_args();
    let puzzles = parse_puzzles(PUZZLE_STR);

    let threads = match args.threads {
        Some(n) => n,
        None => num_cpus::get(),
    };
    eprintln!("starting benchmark with {} threads...", threads);
    if threads == 0 {
        let mut measurements = vec![];
        measurements.extend(run_benchmark(puzzles));
        let mut benchmark = BenchmarkReport::new();
        for m in measurements.iter() {
            benchmark.add_measurement(m);
        }
        println!("{}", benchmark);
    } else {
        let measurements = Arc::new(Mutex::new(vec![]));
        let puzzle_groups = round_robin_split(puzzles, threads);
        let mut handles = vec![];
        for puzzles in puzzle_groups.into_iter() {
            let measurements = measurements.clone();
            handles.push(thread::spawn(move || {
                let ms = run_benchmark(puzzles);
                measurements.lock().unwrap().extend(ms);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let mut benchmark = BenchmarkReport::new();
        for m in measurements.lock().unwrap().iter() {
            benchmark.add_measurement(m);
        }
        println!("{}", benchmark);
    }
}
