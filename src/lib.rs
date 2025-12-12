mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use crate::{
    day::Day, day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7,
    day8::Day8, day9::Day9,
};

pub fn run(lines: Vec<String>, day: &str) {
    match day {
        "day1" => run_day::<Day1>(lines),
        "day2" => run_day::<Day2>(lines),
        "day3" => run_day::<Day3>(lines),
        "day4" => run_day::<Day4>(lines),
        "day5" => run_day::<Day5>(lines),
        "day6" => run_day::<Day6>(lines),
        "day7" => run_day::<Day7>(lines),
        "day8" => run_day::<Day8>(lines),
        "day9" => run_day::<Day9>(lines),
        _ => panic!("Day [{}] isn't supported", day),
    }
}

fn run_day<T: Day>(lines: Vec<String>) {
    let res_a = T::part_a(&lines[..]);
    let res_b = T::part_b(&lines[..]);

    println!("a: {}", res_a);
    println!("b: {}", res_b);
}
