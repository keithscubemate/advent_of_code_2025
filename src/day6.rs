//! Day 6: Column Arithmetic
//!
//! Performs column-wise arithmetic operations on numeric data. Each column
//! has an associated operation (+ or *) that is applied to reduce the column
//! to a single value.
//!
//! ## Input Format
//! Rows of space-separated numbers, with the last row containing operations (+ or *).
//!
//! ## Part A
//! Parses columns by whitespace, applies operations, and sums the results.
//!
//! ## Part B
//! Uses fixed-width columns based on operation positions, reads digits vertically
//! to form numbers, then applies operations and sums the results.

use anyhow::{Error, Result, anyhow};

use crate::day::Day;
use std::str::FromStr;

/// Solution for Day 6: Column Arithmetic puzzle.
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

/// Arithmetic operation type for column reduction.
#[derive(Debug)]
enum Operand {
    /// Addition operation (+)
    Plus,
    /// Multiplication operation (*)
    Times,
}

impl Operand {
    /// Applies the operation to two operands.
    fn apply<'a, 'b>(&'a self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
        }
    }

    /// Returns the identity value for this operation (0 for +, 1 for *).
    fn init(&self) -> u64 {
        match self {
            Self::Plus => 0,
            Self::Times => 1,
        }
    }

    /// Parses an operand from a single character.
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

/// Parses the operation line to extract operations and their column widths.
///
/// Scans the line for `+` and `*` characters, tracking the number of
/// characters between operations to determine column widths.
///
/// # Returns
/// A vector of (operation, width) pairs for each column.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_plus_apply() {
        let op = Operand::Plus;
        assert_eq!(op.apply(3, 5), 8);
        assert_eq!(op.apply(0, 10), 10);
    }

    #[test]
    fn test_operand_times_apply() {
        let op = Operand::Times;
        assert_eq!(op.apply(3, 5), 15);
        assert_eq!(op.apply(0, 10), 0);
        assert_eq!(op.apply(1, 10), 10);
    }

    #[test]
    fn test_operand_init() {
        assert_eq!(Operand::Plus.init(), 0);
        assert_eq!(Operand::Times.init(), 1);
    }

    #[test]
    fn test_operand_from_str() {
        assert!(matches!(Operand::from_str("+"), Ok(Operand::Plus)));
        assert!(matches!(Operand::from_str("*"), Ok(Operand::Times)));
        assert!(Operand::from_str("x").is_err());
    }

    #[test]
    fn test_operand_from_char() {
        assert!(matches!(Operand::from_char('+'), Ok(Operand::Plus)));
        assert!(matches!(Operand::from_char('*'), Ok(Operand::Times)));
        assert!(Operand::from_char('x').is_err());
    }

    #[test]
    fn test_op_and_width_single() {
        let result = op_and_width("+");
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0].0, Operand::Plus));
        assert_eq!(result[0].1, 0);
    }

    #[test]
    fn test_op_and_width_with_spaces() {
        // "  +  *" -> first op found is '+' at position 2
        // When '+' is found, nothing pushed yet (op was None), then count resets
        // When '*' is found, push (Plus, 2) from chars between + and *, then count resets
        // End: push (Times, 0)
        let result = op_and_width("  +  *");
        assert_eq!(result.len(), 2);
        assert!(matches!(result[0].0, Operand::Plus));
        assert_eq!(result[0].1, 2); // 2 spaces between + and *
        assert!(matches!(result[1].0, Operand::Times));
        assert_eq!(result[1].1, 0); // no chars after *
    }

    #[test]
    fn test_part_a_simple_addition() {
        let input = vec!["1 2".to_string(), "3 4".to_string(), "+ +".to_string()];
        // Column 0: 1 + 3 = 4
        // Column 1: 2 + 4 = 6
        // Sum: 4 + 6 = 10
        assert_eq!(Day6::part_a(&input), "10");
    }

    #[test]
    fn test_part_a_simple_multiplication() {
        let input = vec!["2 3".to_string(), "4 5".to_string(), "* *".to_string()];
        // Column 0: 2 * 4 = 8
        // Column 1: 3 * 5 = 15
        // Sum: 8 + 15 = 23
        assert_eq!(Day6::part_a(&input), "23");
    }

    #[test]
    fn test_part_a_mixed_operations() {
        let input = vec!["1 2".to_string(), "3 4".to_string(), "+ *".to_string()];
        // Column 0 (+): 1 + 3 = 4
        // Column 1 (*): 2 * 4 = 8
        // Sum: 4 + 8 = 12
        assert_eq!(Day6::part_a(&input), "12");
    }

    #[test]
    fn test_part_a_single_row() {
        let input = vec!["5 10".to_string(), "+ *".to_string()];
        // Column 0 (+): init=0, fold with 5 = 5
        // Column 1 (*): init=1, fold with 10 = 10
        // Sum: 5 + 10 = 15
        assert_eq!(Day6::part_a(&input), "15");
    }

    #[test]
    fn test_part_a_three_rows() {
        let input = vec![
            "1 1".to_string(),
            "2 2".to_string(),
            "3 3".to_string(),
            "+ *".to_string(),
        ];
        // Column 0 (+): 1 + 2 + 3 = 6
        // Column 1 (*): 1 * 2 * 3 = 6
        // Sum: 6 + 6 = 12
        assert_eq!(Day6::part_a(&input), "12");
    }
}
