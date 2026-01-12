//! Day 7: Beam Splitting
//!
//! Simulates a beam traveling through a grid that splits when hitting obstacles.
//! The beam starts at position `S` and splits into two beams (left and right)
//! whenever it encounters an obstacle `^`.
//!
//! ## Input Format
//! A grid where `.` is empty, `S` is the beam start, and `^` is an obstacle.
//!
//! ## Part A
//! Counts the total number of splits that occur as beams propagate.
//!
//! ## Part B
//! Counts the total number of beams at the end of the simulation,
//! tracking beam counts through splits.

use crate::day::Day;

/// Solution for Day 7: Beam Splitting puzzle.
pub struct Day7 {}

impl Day for Day7 {
    fn part_a(lines: &[String]) -> String {
        let mut current: Vec<Tile> = lines
            .first()
            .unwrap()
            .chars()
            .map(|c| Tile::from_char(c))
            .collect();

        let obs_lines = lines[1..]
            .into_iter()
            .filter(|l| l.contains('^'))
            .map(|l| l.chars().map(|c| Tile::from_char(c)).collect::<Vec<_>>());

        let mut split = 0;

        for line in obs_lines {
            let mut temp = vec![Tile::Empty; current.len()];
            for (i, tile) in line.into_iter().enumerate() {
                if current[i] != Tile::Beam {
                    continue;
                }

                if tile != Tile::Obstacle {
                    temp[i] = Tile::Beam;
                    continue;
                }

                temp[i - 1] = Tile::Beam;
                temp[i + 1] = Tile::Beam;
                split += 1;
            }

            for i in 0..current.len() {
                current[i] = temp[i];
            }
        }

        split.to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let mut current: Vec<u64> = lines
            .first()
            .unwrap()
            .chars()
            .map(|c| match c {
                'S' => 1,
                _ => 0,
            })
            .collect();

        let obs_lines = lines[1..]
            .into_iter()
            .filter(|l| l.contains('^'))
            .map(|l| l.chars().map(|c| Tile::from_char(c)).collect::<Vec<_>>());

        for line in obs_lines {
            let mut temp = vec![0; current.len()];

            println!("{:?}", current);

            for (i, tile) in line.into_iter().enumerate() {
                if current[i] == 0 {
                    continue;
                }

                if tile != Tile::Obstacle {
                    temp[i] += current[i];
                    continue;
                }

                temp[i - 1] += current[i];
                temp[i + 1] += current[i];
            }

            for i in 0..current.len() {
                current[i] = temp[i];
            }
        }

        current.into_iter().sum::<u64>().to_string()
    }
}

/// Represents a cell type in the beam grid.
#[derive(PartialEq, Clone, Copy)]
enum Tile {
    /// Empty space (`.`) - beams pass through
    Empty,
    /// Beam position (`S`) - indicates beam presence
    Beam,
    /// Obstacle (`^`) - causes beams to split
    Obstacle,
}

impl Tile {
    /// Parses a tile from its character representation.
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::Beam,
            '^' => Self::Obstacle,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_char() {
        assert!(matches!(Tile::from_char('.'), Tile::Empty));
        assert!(matches!(Tile::from_char('S'), Tile::Beam));
        assert!(matches!(Tile::from_char('^'), Tile::Obstacle));
    }

    #[test]
    #[should_panic]
    fn test_tile_from_char_invalid() {
        Tile::from_char('X');
    }

    #[test]
    fn test_part_a_no_obstacles() {
        let input = vec![".S.".to_string(), "...".to_string()];
        // No obstacle lines (no '^'), so no splits
        assert_eq!(Day7::part_a(&input), "0");
    }

    #[test]
    fn test_part_a_single_split() {
        let input = vec![".S.".to_string(), ".^.".to_string()];
        // Beam starts at position 1
        // Obstacle at position 1 in second row
        // Beam hits obstacle, splits to positions 0 and 2
        assert_eq!(Day7::part_a(&input), "1");
    }

    #[test]
    fn test_part_a_beam_misses_obstacle() {
        let input = vec!["S..".to_string(), "..^".to_string()];
        // Beam at position 0
        // Obstacle at position 2, beam doesn't hit it
        assert_eq!(Day7::part_a(&input), "0");
    }

    #[test]
    fn test_part_a_multiple_obstacles() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // First row: beam at position 2
        // Second row: obstacle at 2, beam splits to 1 and 3 (split count = 1)
        // Third row: obstacles at 1 and 3, both beams split (split count += 2 = 3)
        assert_eq!(Day7::part_a(&input), "3");
    }

    #[test]
    fn test_part_b_no_obstacles() {
        let input = vec![".S.".to_string(), "...".to_string()];
        // No obstacle lines, beam count stays at 1
        assert_eq!(Day7::part_b(&input), "1");
    }

    #[test]
    fn test_part_b_single_split() {
        let input = vec![".S.".to_string(), ".^.".to_string()];
        // Beam starts with count 1 at position 1
        // Hits obstacle, splits to positions 0 and 2, each with count 1
        // Total: 1 + 1 = 2
        assert_eq!(Day7::part_b(&input), "2");
    }

    #[test]
    fn test_part_b_cascading_splits() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Start: position 2 has count 1
        // After row 2: positions 1 and 3 each have count 1
        // After row 3: positions 0, 2, 2, 4 each get count 1
        // Total: 4 beams
        assert_eq!(Day7::part_b(&input), "4");
    }

    #[test]
    fn test_part_b_no_split_pass_through() {
        let input = vec!["S..".to_string(), "...".to_string(), "...".to_string()];
        // Beam passes through without hitting obstacles
        assert_eq!(Day7::part_b(&input), "1");
    }
}
