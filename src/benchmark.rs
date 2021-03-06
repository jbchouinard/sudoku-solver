use std::collections::HashMap;
use std::str::FromStr;

use sudoku::solver::strategies::all_strategies;
use sudoku::solver::{Solution, Solver};
use sudoku::{Error, Grid, Result};

const EASY_PUZZLES_STR: &str = include_str!("../puzzles/easy.txt");
const MEDIUM_PUZZLES_STR: &str = include_str!("../puzzles/medium.txt");
const HARD_PUZZLES_STR: &str = include_str!("../puzzles/hard.txt");
const DIABOLICAL_PUZZLES_STR: &str = include_str!("../puzzles/diabolical.txt");

struct Puzzle {
    id: String,
    grid: Grid,
    rating: f64,
}

impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err(Error::new("Invalid puzzle format"));
        }
        let id = parts[0];
        let puzzle_str = parts[1];
        let rating = parts[2];
        let grid = Grid::from_str(puzzle_str)?;
        Ok(Puzzle {
            id: id.to_string(),
            grid,
            rating: rating.parse().unwrap(),
        })
    }
}

fn parse_puzzles(s: &str) -> Result<HashMap<String, Puzzle>> {
    let mut map: HashMap<String, Puzzle> = HashMap::new();
    for puzzle_str in s.split('\n') {
        let puzzle = Puzzle::from_str(puzzle_str)?;
        map.insert(puzzle.id.clone(), puzzle);
    }
    Ok(map)
}

struct RunningAverage {
    count: f64,
    sum: f64,
}

impl RunningAverage {
    fn new() -> Self {
        Self {
            count: 0.0,
            sum: 0.0,
        }
    }
    fn update(&mut self, value: f64) {
        self.count += 1.0;
        self.sum += value;
    }
    fn average(&self) -> f64 {
        self.sum / self.count
    }
}

struct Benchmark {
    total: u64,
    solved: u64,
    hardest_rating: f64,
    hardest_rating_solved: f64,
    avg_steps: RunningAverage,
    avg_steps_solved: RunningAverage,
    avg_time: RunningAverage,
    avg_time_solved: RunningAverage,
}

impl Benchmark {
    fn new() -> Self {
        Benchmark {
            total: 0,
            solved: 0,
            hardest_rating: 0.0,
            hardest_rating_solved: 0.0,
            avg_steps: RunningAverage::new(),
            avg_steps_solved: RunningAverage::new(),
            avg_time: RunningAverage::new(),
            avg_time_solved: RunningAverage::new(),
        }
    }
    fn total_time(sol: &Solution) -> f64 {
        let mut total: f64 = 0.0;
        for step in &sol.steps {
            total += step.time.as_micros() as f64;
        }
        total
    }
    fn add(&mut self, puz: &Puzzle, sol: &Solution) {
        self.total += 1;
        self.hardest_rating = self.hardest_rating.max(puz.rating);
        self.avg_steps.update(sol.steps.len() as f64);
        self.avg_time.update(Self::total_time(sol));
        if sol.grid.is_solved() {
            self.solved += 1;
            self.hardest_rating_solved = self.hardest_rating_solved.max(puz.rating);
            self.avg_steps_solved.update(sol.steps.len() as f64);
            self.avg_time_solved.update(Self::total_time(sol));
        }
    }
    fn print_summary(&self) {
        println!(
            "Solved {} of {} puzzles ({:.1}%)",
            self.solved,
            self.total,
            100 * self.solved / self.total
        );
        println!("All puzzles");
        println!("Hardest rating: {:.1}", self.hardest_rating);
        println!("Average steps: {:.1}", self.avg_steps.average());
        println!("Average time: {:.0} μs", self.avg_time.average());
        println!("Solved puzzles");
        println!("Hardest rating: {:.1}", self.hardest_rating_solved);
        println!("Average steps: {:.1}", self.avg_steps_solved.average());
        println!("Average time: {:.0} μs", self.avg_time_solved.average());
    }
}

fn run_benchmark(puzzles_str: &str) -> Benchmark {
    let solver = Solver::new(all_strategies());
    let puzzles = parse_puzzles(puzzles_str).unwrap();
    let mut benchmark = Benchmark::new();
    for (_, puz) in puzzles {
        let sol = solver.solve(&puz.grid);
        benchmark.add(&puz, &sol);
    }
    benchmark
}

fn main() {
    println!("\nEASY PUZZLES...");
    run_benchmark(EASY_PUZZLES_STR).print_summary();
    println!("\nMEDIUM PUZZLES...");
    run_benchmark(MEDIUM_PUZZLES_STR).print_summary();
    println!("\nHARD PUZZLES...");
    run_benchmark(HARD_PUZZLES_STR).print_summary();
    println!("\nDIABOLICAL PUZZLES...");
    run_benchmark(DIABOLICAL_PUZZLES_STR).print_summary();
}
