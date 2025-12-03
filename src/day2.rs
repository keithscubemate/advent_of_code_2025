use crate::day::Day;

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

fn is_repeat(num: u64) -> bool {
    let num = num.to_string();
    for len in 2..=num.len() {
        if is_repeat_n(&num, len) {
            return true;
        }
    }
    false
}

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
