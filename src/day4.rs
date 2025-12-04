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
