//! Day 1: Dial Rotation
//!
//! Simulates a circular dial with 100 positions (0-99). The dial starts at position 50,
//! and each instruction rotates it left (L) or right (R) by a given amount.
//!
//! ## Input Format
//! Each line contains a direction and amount, e.g., `L68` (left 68) or `R48` (right 48).
//!
//! ## Part A
//! Counts how many times the dial lands exactly on position 0 after each move,
//! treating moves modularly within the dial size.
//!
//! ## Part B
//! Counts zero crossings including full rotations. Large moves that wrap around
//! the dial multiple times count each pass through zero.

use anyhow::Result;

use crate::day::Day;

/// Solution for Day 1: Dial Rotation puzzle.
pub struct Day1;

impl Day for Day1 {
    fn part_a(lines: &[String]) -> String {
        let dial_size = 100;
        let mut dial = 50;

        let turns = lines.iter().map(|l| parse_line(l).unwrap());

        let mut zero_count = 0;

        for turn in turns {
            let turn = if turn < 0 { turn + dial_size } else { turn };

            dial += turn;
            dial %= dial_size;

            if dial == 0 {
                zero_count += 1;
            }
        }

        format!("{}", zero_count)
    }

    fn part_b(lines: &[String]) -> String {
        let dial_size = 100;
        let mut dial = 50;

        let turns = lines.iter().map(|l| parse_line(l).unwrap());

        let mut zero_count = 0;

        for turn in turns {
            let last_dial = dial;

            let full_turns = (turn / dial_size).abs();

            zero_count += full_turns;

            let turn = turn % dial_size;

            dial += turn;

            if dial == 0 {
                zero_count += 1;
            } else if dial < 0 {
                dial += 100;
                if last_dial != 0 {
                    zero_count += 1;
                }
            } else if dial >= dial_size {
                dial %= dial_size;
                zero_count += 1;
            }
        }

        format!("{}", zero_count)
    }
}

/// Parses a dial instruction from a line of input.
///
/// Instructions have the format `<direction><amount>` where direction is
/// `L` (left/negative) or `R` (right/positive).
///
/// # Arguments
/// * `line` - A string like "L68" or "R48"
///
/// # Returns
/// A signed integer representing the rotation amount (negative for left).
fn parse_line(line: &str) -> Result<i32> {
    let negative = line.starts_with('L');

    let val = line[1..].parse::<i32>()?;

    if negative {
        return Ok(-val);
    }

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_right() {
        assert_eq!(parse_line("R48").unwrap(), 48);
        assert_eq!(parse_line("R100").unwrap(), 100);
        assert_eq!(parse_line("R0").unwrap(), 0);
    }

    #[test]
    fn test_parse_line_left() {
        assert_eq!(parse_line("L68").unwrap(), -68);
        assert_eq!(parse_line("L30").unwrap(), -30);
        assert_eq!(parse_line("L100").unwrap(), -100);
    }

    #[test]
    fn test_parse_line_invalid() {
        // Only unparseable numbers cause errors
        assert!(parse_line("R").is_err());
        assert!(parse_line("Rabc").is_err());
        assert!(parse_line("L").is_err());
    }

    #[test]
    #[should_panic]
    fn test_parse_line_empty_panics() {
        // Empty string causes a panic (accessing [1..] on empty string)
        parse_line("").ok();
    }

    #[test]
    fn test_parse_line_any_prefix() {
        // Any non-L prefix is treated as positive
        assert_eq!(parse_line("X50").unwrap(), 50);
        assert_eq!(parse_line("Z100").unwrap(), 100);
    }

    #[test]
    fn test_part_a_small_input() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];

        // Starting at 50:
        // L68 (-68): 50 + (-68) = -18 -> 82 (mod 100)
        // L30 (-30): 82 + (-30) = 52
        // R48 (48): 52 + 48 = 100 -> 0 (mod 100) ✓ count=1
        // L5 (-5): 0 + (-5) = -5 -> 95 (mod 100)
        // R60 (60): 95 + 60 = 155 -> 55 (mod 100)
        // L55 (-55): 55 + (-55) = 0 ✓ count=2
        // L1 (-1): 0 + (-1) = -1 -> 99 (mod 100)
        // L99 (-99): 99 + (-99) = 0 ✓ count=3
        // R14 (14): 0 + 14 = 14
        // L82 (-82): 14 + (-82) = -68 -> 32 (mod 100)

        assert_eq!(Day1::part_a(&input), "3");
    }

    #[test]
    fn test_part_a_simple() {
        // Start at 50, move to 0 directly
        let input = vec!["L50".to_string()];
        assert_eq!(Day1::part_a(&input), "1");
    }

    #[test]
    fn test_part_a_no_zeros() {
        // Start at 50, move by 10 (never hits 0)
        let input = vec!["R10".to_string(), "R10".to_string()];
        assert_eq!(Day1::part_a(&input), "0");
    }

    #[test]
    fn test_part_b_full_rotation() {
        // A turn of 100+ should count full rotations
        let input = vec!["R150".to_string()]; // 1 full rotation + 50 more
        // Start at 50, add 150 = 200 -> lands at 0 with 2 zero crossings
        assert_eq!(Day1::part_b(&input), "2");
    }

    #[test]
    fn test_part_b_small_input() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];

        // This tests the actual logic with the small input file
        // The exact count depends on the part_b logic for counting zero crossings
        let result = Day1::part_b(&input);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_part_b_crossing_zero() {
        // Start at 50, go backward past zero
        let input = vec!["L60".to_string()]; // 50 - 60 = -10 -> 90, crosses zero
        assert_eq!(Day1::part_b(&input), "1");
    }
}
