use advent_of_code_2025::{day::Day, day1::Day1};
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

    let day = match args.day.as_str() {
        "day1" => run::<Day1>(lines),
        _ => panic!(),
    };
}

fn run<T: Day>(lines: Vec<String>) {
    let res_a = T::part_a(&lines[..]);
    let res_b = T::part_b(&lines[..]);

    println!("a: {}", res_a);
    println!("b: {}", res_b);
}
