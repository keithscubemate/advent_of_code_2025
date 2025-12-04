use advent_of_code_2025::run;
use std::io;

use clap::Parser;

/// Run a specific day of AOC
#[derive(Parser)]
struct Cli {
    /// The day to run
    day: String,
}

fn main() {
    let args = Cli::parse();

    let stdin = io::stdin();

    let lines = stdin.lines();

    let lines = lines.map(|l| l.unwrap()).collect::<Vec<String>>();

    let day = args.day.as_str();

    run(lines, day);
}
