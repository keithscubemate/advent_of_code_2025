use crate::day::Day;

pub struct Day4 {}

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    Bale,
}

impl Day for Day4 {
    fn part_a(lines: &[String]) -> String {
        let grid: Vec<Vec<_>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Square::Empty,
                        '@' => Square::Bale,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        let height = grid.len();
        let width = grid[0].len();

        let mut accessable_coords = 0;

        for i in 0..height {
            for j in 0..width {
                if is_accessable(i as isize, j as isize, &grid, 4) {
                    accessable_coords += 1;
                }
            }
        }

        accessable_coords.to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let mut grid: Vec<Vec<_>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Square::Empty,
                        '@' => Square::Bale,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        let height = grid.len();
        let width = grid[0].len();

        let mut accessable_coords = 0;

        loop {
            let mut bales_to_remove = vec![];

            for i in 0..height {
                for j in 0..width {
                    if is_accessable(i as isize, j as isize, &grid, 4) {
                        accessable_coords += 1;
                        bales_to_remove.push((i, j));
                    }
                }
            }

            if bales_to_remove.is_empty() {
                break;
            }

            for (bi, bj) in bales_to_remove {
                grid[bi][bj] = Square::Empty;
            }
        }

        accessable_coords.to_string()
    }
}

fn is_accessable(i: isize, j: isize, grid: &Vec<Vec<Square>>, bale_limit: usize) -> bool {
    if i as usize >= grid.len() || j as usize >= grid[0].len() {
        return false;
    }

    if grid[i as usize][j as usize] != Square::Bale {
        return false;
    }

    let directions = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    let mut bales = 0;

    for (di, dj) in directions {
        let ni = (i + di) as usize;
        let nj = (j + dj) as usize;

        if ni as usize >= grid.len() || nj as usize >= grid[0].len() {
            continue;
        }

        let neighbor = &grid[ni as usize][nj as usize];

        if *neighbor == Square::Bale {
            bales += 1;
        }
    }

    bales < bale_limit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(lines: &[&str]) -> Vec<Vec<Square>> {
        lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Square::Empty,
                        '@' => Square::Bale,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_is_accessable_isolated_bale() {
        let grid = make_grid(&["...", ".@.", "..."]);
        // Isolated bale has 0 neighbors, should be accessible with any bale_limit > 0
        assert!(is_accessable(1, 1, &grid, 4));
        assert!(is_accessable(1, 1, &grid, 1));
    }

    #[test]
    fn test_is_accessable_surrounded_bale() {
        let grid = make_grid(&["@@@", "@@@", "@@@"]);
        // Center bale has 8 neighbors
        assert!(!is_accessable(1, 1, &grid, 4)); // 8 >= 4, not accessible
        assert!(!is_accessable(1, 1, &grid, 8)); // 8 >= 8, not accessible
        assert!(is_accessable(1, 1, &grid, 9));  // 8 < 9, accessible
    }

    #[test]
    fn test_is_accessable_corner_bale() {
        let grid = make_grid(&["@.", ".."]);
        // Corner bale at (0,0) has only 3 possible neighbors, all empty
        assert!(is_accessable(0, 0, &grid, 4));
        assert!(is_accessable(0, 0, &grid, 1));
    }

    #[test]
    fn test_is_accessable_empty_square() {
        let grid = make_grid(&["...", ".@.", "..."]);
        // Empty square is never accessible (must be a bale)
        assert!(!is_accessable(0, 0, &grid, 4));
    }

    #[test]
    fn test_is_accessable_out_of_bounds() {
        let grid = make_grid(&["@"]);
        assert!(!is_accessable(-1, 0, &grid, 4));
        assert!(!is_accessable(0, -1, &grid, 4));
        assert!(!is_accessable(1, 0, &grid, 4));
        assert!(!is_accessable(0, 1, &grid, 4));
    }

    #[test]
    fn test_is_accessable_partial_neighbors() {
        let grid = make_grid(&["@@.", "@..", "..."]);
        // Bale at (0,0) has 2 bale neighbors: (0,1) and (1,0)
        assert!(is_accessable(0, 0, &grid, 4)); // 2 < 4
        assert!(is_accessable(0, 0, &grid, 3)); // 2 < 3
        assert!(!is_accessable(0, 0, &grid, 2)); // 2 >= 2
    }

    #[test]
    fn test_part_a_simple() {
        let input = vec![
            "..@..".to_string(),
            ".@@@.".to_string(),
            "..@..".to_string(),
        ];
        // Cross pattern: center has 4 neighbors, edges have 1 neighbor each
        // With bale_limit=4: center (4 neighbors) is not accessible, 4 edges (1 neighbor each) are accessible
        assert_eq!(Day4::part_a(&input), "4");
    }

    #[test]
    fn test_part_a_all_isolated() {
        let input = vec![
            "@.@".to_string(),
            "...".to_string(),
            "@.@".to_string(),
        ];
        // 4 isolated bales, all accessible
        assert_eq!(Day4::part_a(&input), "4");
    }

    #[test]
    fn test_part_a_dense_grid() {
        let input = vec![
            "@@@".to_string(),
            "@@@".to_string(),
            "@@@".to_string(),
        ];
        // 3x3 grid of bales
        // Corners have 3 neighbors, edges have 5, center has 8
        // With limit 4: only corners (3 < 4) are accessible
        assert_eq!(Day4::part_a(&input), "4");
    }

    #[test]
    fn test_part_b_cascading_removal() {
        let input = vec![
            "@@@".to_string(),
            "@@@".to_string(),
            "@@@".to_string(),
        ];
        // First pass: remove 4 corners
        // Second pass: the remaining bales become accessible
        // All 9 bales should be counted
        assert_eq!(Day4::part_b(&input), "9");
    }

    #[test]
    fn test_part_b_no_bales() {
        let input = vec![
            "...".to_string(),
            "...".to_string(),
        ];
        assert_eq!(Day4::part_b(&input), "0");
    }
}
