use crate::day::Day;

pub struct Day3 {}

impl Day for Day3 {
    fn part_a(lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| (c as u64) - ('0' as u64))
                    .collect::<Vec<_>>()
            })
            .map(|b| get_joltage(&b, 2))
            .sum::<u64>()
            .to_string()
    }

    fn part_b(lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| (c as u64) - ('0' as u64))
                    .collect::<Vec<_>>()
            })
            .map(|b| get_joltage(&b, 12))
            .sum::<u64>()
            .to_string()
    }
}

fn get_joltage(bank: &[u64], digits_to_find: usize) -> u64 {
    let mut digits = vec![];

    let mut idx = 0;

    for i in 0..digits_to_find {
        let start = idx;
        let end = bank.len() - (digits_to_find - i - 1);

        let bank_slice = &bank[start..end];

        let (i, new_digit) = max_of_range_with_idx(bank_slice, idx);

        idx = i + 1;

        digits.push(new_digit);
    }

    digits.iter().fold(0, |acc, val| (acc * 10 + val))
}

fn max_of_range_with_idx(bank: &[u64], start: usize) -> (usize, u64) {
    let mut max_idx = 0;
    let mut max = bank[max_idx];

    for i in max_idx + 1..bank.len() {
        let val = bank[i];

        if val > max {
            max = val;
            max_idx = i;
        }
    }

    (max_idx + start, max)
}
