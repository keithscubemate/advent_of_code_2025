use anyhow::{Error, Result, anyhow};

use crate::day::Day;
use std::str::FromStr;

pub struct Day6 {}

impl Day for Day6 {
    fn part_a(lines: &[String]) -> String {
        let operands: Vec<Operand> = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| Operand::from_str(s).unwrap())
            .collect();

        let col_count = operands.len();

        let mut vals = vec![vec![]; col_count];

        for line in &lines[..lines.len() - 1] {
            let new_vals = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());

            for (i, val) in new_vals.enumerate() {
                vals[i].push(val);
            }
        }

        let mut rv: Vec<u64> = vec![];
        for (op, vals) in operands.iter().zip(vals.iter()) {
            let init = op.init();
            let val = vals.into_iter().fold(init, |a, b| op.apply(a, *b));
            rv.push(val);
        }

        rv.into_iter().sum::<u64>().to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let ops_and_widths = op_and_width(lines.last().unwrap());

        println!("{:?}", ops_and_widths);

        let col_count = ops_and_widths.len();

        let mut vals = vec![vec![]; col_count];

        for line in &lines[..lines.len() - 1] {
            let mut total_len = 0;
            for i in 0..col_count {
                let col_width = ops_and_widths[i].1;

                let val = if i == col_count - 1 {
                    &line[total_len..]
                } else {
                    &line[total_len..total_len + col_width]
                };

                vals[i].push(val);
                total_len += col_width + 1;
            }
        }

        let mut numbers = vec![];

        for val in vals {
            let mut nums = vec![];

            let max_len = val.iter().map(|v| v.len()).max().unwrap();

            for i in 0..max_len {
                let num = val
                    .iter()
                    .map(|v| {
                        v.chars()
                            .nth(i)
                            .filter(|c| matches!(c, '0'..='9'))
                            .map(|c| (c as u64) - ('0' as u64))
                    })
                    .fold(0, |acc, x| match x {
                        Some(x) => acc * 10 + x,
                        None => acc,
                    });
                nums.push(num)
            }
            numbers.push(nums)
        }

        let operands = ops_and_widths.into_iter().map(|(o, _)| o);

        let mut rv: Vec<u64> = vec![];
        for (op, vals) in operands.zip(numbers.iter()) {
            let init = op.init();
            let val = vals.into_iter().fold(init, |a, b| op.apply(a, *b));
            rv.push(val);
        }

        rv.into_iter().sum::<u64>().to_string()
    }
}

#[derive(Debug)]
enum Operand {
    Plus,
    Times,
}

impl Operand {
    fn apply<'a, 'b>(&'a self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
        }
    }

    fn init(&self) -> u64 {
        match self {
            Self::Plus => 0,
            Self::Times => 1,
        }
    }

    fn from_char(c: char) -> Result<Self> {
        match c {
            '+' => Ok(Self::Plus),
            '*' => Ok(Self::Times),
            c => Err(anyhow!("Frick: [{}]", c)),
        }
    }
}

impl FromStr for Operand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Times),
            _ => Err(anyhow!("Frick")),
        }
    }
}

fn op_and_width(line: &str) -> Vec<(Operand, usize)> {
    let mut rv = vec![];
    let mut count = 0;
    let mut op = None;

    for c in line.chars() {
        if !"+*".contains(c) {
            count += 1;
            continue;
        }

        if let Some(good_op) = op {
            rv.push((good_op, count));
        }

        op = Some(Operand::from_char(c).unwrap());
        count = 0;
    }

    rv.push((op.take().unwrap(), count));

    rv
}
