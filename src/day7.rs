use crate::day::Day;

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

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Beam,
    Obstacle,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::Beam,
            '^' => Self::Obstacle,
            _ => panic!(),
        }
    }
}
