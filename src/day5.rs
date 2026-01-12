//! Day 5: Range Filtering
//!
//! Works with numeric ranges and ID filtering. Uses interval merging
//! techniques to efficiently count covered values.
//!
//! ## Input Format
//! Ranges in `start-end` format, followed by a blank line, then IDs to check.
//!
//! ## Part A
//! Counts how many IDs fall within at least one of the given ranges.
//!
//! ## Part B
//! Calculates the total count of unique values covered by all ranges
//! (merging overlapping ranges).

use crate::day::Day;

/// Solution for Day 5: Range Filtering puzzle.
pub struct Day5 {}

impl Day for Day5 {
    fn part_a(lines: &[String]) -> String {
        let split = lines.iter().position(|l| l.is_empty()).unwrap();
        let r_lines = &lines[..split];
        let id_lines = &lines[split + 1..];

        let ranges: Vec<_> = r_lines
            .iter()
            .map(|l| {
                let mut vals = l.split("-");

                let start = vals.next().unwrap().parse::<u64>().unwrap();
                let end = vals.next().unwrap().parse::<u64>().unwrap();

                start..=end
            })
            .collect();

        let ids: Vec<u64> = id_lines.iter().map(|l| l.parse::<u64>().unwrap()).collect();

        ids.iter()
            .filter(|id| ranges.iter().any(|r| r.contains(id)))
            .count()
            .to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let split = lines.iter().position(|l| l.is_empty()).unwrap();
        let r_lines = &lines[..split];

        let mut boundaries: Vec<Boundary> = vec![];

        for line in r_lines {
            let mut vals = line.split("-");

            let start = vals.next().unwrap().parse::<u64>().unwrap();
            let end = vals.next().unwrap().parse::<u64>().unwrap();

            let start = Boundary {
                value: start,
                side: Side::Start,
            };

            let end = Boundary {
                value: end,
                side: Side::End,
            };

            boundaries.push(start);
            boundaries.push(end);
        }

        boundaries.sort();

        let mut starts_seen = 0;
        let mut last_start = None;
        let mut fresh_ids = 0;
        for boundary in boundaries {
            match boundary.side {
                Side::Start => {
                    if last_start.is_none() {
                        last_start = Some(boundary.value);
                    }
                    starts_seen += 1;
                }
                Side::End => {
                    starts_seen -= 1;

                    if starts_seen > 0 {
                        continue;
                    }

                    let ls = last_start.take().unwrap();

                    let new_end = boundary.value;

                    fresh_ids += 1 + new_end - ls;
                }
            }
        }

        fresh_ids.to_string()
    }
}

/// Indicates whether a boundary is the start or end of a range.
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
enum Side {
    /// Beginning of a range (sorted before End at same value)
    Start,
    /// End of a range
    End,
}

/// Represents a boundary point of a range for interval merging.
///
/// Boundaries are sorted by value first, then by side (Start before End).
/// This ordering ensures proper merging of adjacent and overlapping ranges.
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
struct Boundary {
    /// The numeric position of this boundary
    value: u64,
    /// Whether this is a range start or end
    side: Side,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_single_range_single_id_match() {
        let input = vec![
            "1-10".to_string(),
            "".to_string(),
            "5".to_string(),
        ];
        assert_eq!(Day5::part_a(&input), "1");
    }

    #[test]
    fn test_part_a_single_range_single_id_no_match() {
        let input = vec![
            "1-10".to_string(),
            "".to_string(),
            "15".to_string(),
        ];
        assert_eq!(Day5::part_a(&input), "0");
    }

    #[test]
    fn test_part_a_multiple_ranges() {
        let input = vec![
            "1-5".to_string(),
            "10-20".to_string(),
            "".to_string(),
            "3".to_string(),
            "7".to_string(),
            "15".to_string(),
        ];
        // 3 is in 1-5, 7 is in neither, 15 is in 10-20
        assert_eq!(Day5::part_a(&input), "2");
    }

    #[test]
    fn test_part_a_boundary_values() {
        let input = vec![
            "5-10".to_string(),
            "".to_string(),
            "5".to_string(),
            "10".to_string(),
            "4".to_string(),
            "11".to_string(),
        ];
        // 5 and 10 are in range (inclusive), 4 and 11 are not
        assert_eq!(Day5::part_a(&input), "2");
    }

    #[test]
    fn test_part_a_overlapping_ranges() {
        let input = vec![
            "1-10".to_string(),
            "5-15".to_string(),
            "".to_string(),
            "7".to_string(),
        ];
        // 7 matches both ranges, but should only count once
        assert_eq!(Day5::part_a(&input), "1");
    }

    #[test]
    fn test_part_b_single_range() {
        let input = vec![
            "1-5".to_string(),
            "".to_string(),
        ];
        // Range 1-5 has 5 fresh IDs
        assert_eq!(Day5::part_b(&input), "5");
    }

    #[test]
    fn test_part_b_non_overlapping_ranges() {
        let input = vec![
            "1-5".to_string(),
            "10-15".to_string(),
            "".to_string(),
        ];
        // 5 + 6 = 11 fresh IDs
        assert_eq!(Day5::part_b(&input), "11");
    }

    #[test]
    fn test_part_b_overlapping_ranges() {
        let input = vec![
            "1-10".to_string(),
            "5-15".to_string(),
            "".to_string(),
        ];
        // Merged range: 1-15 = 15 fresh IDs
        assert_eq!(Day5::part_b(&input), "15");
    }

    #[test]
    fn test_part_b_fully_contained_range() {
        let input = vec![
            "1-20".to_string(),
            "5-10".to_string(),
            "".to_string(),
        ];
        // 5-10 is fully within 1-20, so just 20 fresh IDs
        assert_eq!(Day5::part_b(&input), "20");
    }

    #[test]
    fn test_part_b_adjacent_ranges() {
        let input = vec![
            "1-5".to_string(),
            "6-10".to_string(),
            "".to_string(),
        ];
        // Adjacent but not overlapping: 5 + 5 = 10 fresh IDs
        assert_eq!(Day5::part_b(&input), "10");
    }

    #[test]
    fn test_boundary_ordering() {
        // Test that boundaries sort correctly (by value, then Start before End)
        let b1 = Boundary { value: 5, side: Side::Start };
        let b2 = Boundary { value: 5, side: Side::End };
        let b3 = Boundary { value: 10, side: Side::Start };

        assert!(b1 < b2);
        assert!(b2 < b3);
    }
}
