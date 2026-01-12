//! Day 2: Repeating Number Patterns
//!
//! Analyzes ranges of numbers to find those with specific digit repetition patterns.
//!
//! ## Input Format
//! Comma-separated ranges in the format `start-end`, e.g., `10-100,200-300`.
//!
//! ## Part A
//! Sums all "twice" numbers in the ranges - numbers where the first half of digits
//! equals the second half (e.g., 1212, 77, 123123).
//!
//! ## Part B
//! Sums all "repeat" numbers - numbers that can be divided into 2 or more identical
//! chunks (e.g., 111, 1212, 121212).

use crate::day::Day;

/// Solution for Day 2: Repeating Number Patterns puzzle.
pub struct Day2 {}

impl Day for Day2 {
    fn part_a(lines: &[String]) -> String {
        lines
            .iter()
            .flat_map(|line| line.split(','))
            .filter(|r| !r.is_empty())
            .map(|line| {
                let mut vals = line.split('-').map(|n| n.parse().unwrap());
                (vals.next().unwrap(), vals.next().unwrap())
            })
            .flat_map(|(f, s)| (f..=s).filter(|v| is_twice(*v)))
            .sum::<u64>()
            .to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let ranges: Vec<&str> = lines
            .iter()
            .flat_map(|line| line.split(','))
            .filter(|r| !r.is_empty())
            .collect();

        let ranges: Vec<(u64, u64)> = ranges
            .into_iter()
            .map(|line| {
                let mut vals = line.split('-');
                (vals.next().unwrap(), vals.next().unwrap())
            })
            .map(|(f, s)| (f.parse().unwrap(), s.parse().unwrap()))
            .collect();

        let repeat_counts: u64 = ranges
            .into_iter()
            .flat_map(|(f, s)| (f..=s).filter(|v| is_repeat(*v)))
            .sum();

        format!("{}", repeat_counts)
    }
}

/// Checks if a number is a "twice" number.
///
/// A twice number has an even number of digits where the first half
/// exactly matches the second half (e.g., 77, 1212, 123123).
fn is_twice(num: u64) -> bool {
    let num = num.to_string();

    if (num.len() & 1) == 1 {
        return false;
    }

    let idx = num.len() / 2;
    let left = &num[..idx];
    let right = &num[idx..];

    left == right
}

/// Checks if a number can be split into repeating identical chunks.
///
/// Tries all possible chunk counts (2 to digit length) to find any
/// valid repeating pattern (e.g., 111 splits into "1","1","1").
fn is_repeat(num: u64) -> bool {
    let num = num.to_string();
    for len in 2..=num.len() {
        if is_repeat_n(&num, len) {
            return true;
        }
    }
    false
}

/// Checks if a string can be split into exactly `num_splits` identical chunks.
///
/// Returns false if the string length isn't evenly divisible by `num_splits`.
fn is_repeat_n(num: &str, num_splits: usize) -> bool {
    if num.len() % num_splits != 0 {
        return false;
    }

    let chunk_size = num.len() / num_splits;

    let chars: Vec<char> = num.chars().collect();
    let parts: Vec<_> = chars.chunks(chunk_size).collect();

    let mut part_pairs = parts.windows(2);

    part_pairs.all(|pp| pp[0] == pp[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_twice_true() {
        assert!(is_twice(77));
        assert!(is_twice(1212));
        assert!(is_twice(123123));
        assert!(is_twice(11));
    }

    #[test]
    fn test_is_twice_false() {
        assert!(!is_twice(123)); // odd length
        assert!(!is_twice(1234)); // halves don't match
        assert!(!is_twice(1213)); // close but not equal
        assert!(!is_twice(7));    // single digit
    }

    #[test]
    fn test_is_repeat_n_true() {
        assert!(is_repeat_n("11", 2));
        assert!(is_repeat_n("111", 3));
        assert!(is_repeat_n("1212", 2));
        assert!(is_repeat_n("abcabcabc", 3));
    }

    #[test]
    fn test_is_repeat_n_false() {
        assert!(!is_repeat_n("12", 2));      // chunks don't match
        assert!(!is_repeat_n("123", 2));     // not evenly divisible
        assert!(!is_repeat_n("1213", 2));    // chunks don't match
    }

    #[test]
    fn test_is_repeat_true() {
        assert!(is_repeat(11));
        assert!(is_repeat(111));
        assert!(is_repeat(1212));
        assert!(is_repeat(123123));
        assert!(is_repeat(121212));
    }

    #[test]
    fn test_is_repeat_false() {
        assert!(!is_repeat(12));
        assert!(!is_repeat(123));
        assert!(!is_repeat(1234));
    }
}
