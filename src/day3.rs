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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of_range_with_idx_simple() {
        let bank = vec![1, 5, 3, 2];
        let (idx, max) = max_of_range_with_idx(&bank, 0);
        assert_eq!(idx, 1);
        assert_eq!(max, 5);
    }

    #[test]
    fn test_max_of_range_with_idx_with_offset() {
        let bank = vec![3, 7, 2];
        let (idx, max) = max_of_range_with_idx(&bank, 5);
        assert_eq!(idx, 6); // index 1 in slice + offset 5
        assert_eq!(max, 7);
    }

    #[test]
    fn test_max_of_range_with_idx_first_is_max() {
        let bank = vec![9, 1, 2, 3];
        let (idx, max) = max_of_range_with_idx(&bank, 0);
        assert_eq!(idx, 0);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_max_of_range_with_idx_last_is_max() {
        let bank = vec![1, 2, 3, 9];
        let (idx, max) = max_of_range_with_idx(&bank, 0);
        assert_eq!(idx, 3);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_max_of_range_with_idx_single_element() {
        let bank = vec![5];
        let (idx, max) = max_of_range_with_idx(&bank, 0);
        assert_eq!(idx, 0);
        assert_eq!(max, 5);
    }

    #[test]
    fn test_get_joltage_two_digits() {
        // From digits 3, 9, 1, 8, find the 2 largest in order
        // First: find max in [3,9,1] (leaving room for 1 more) -> 9 at index 1
        // Second: find max in [1,8] -> 8 at index 3
        // Result: 98
        let bank = vec![3, 9, 1, 8];
        assert_eq!(get_joltage(&bank, 2), 98);
    }

    #[test]
    fn test_get_joltage_three_digits() {
        // From digits 1, 2, 9, 3, 7, find 3 largest in order
        let bank = vec![1, 2, 9, 3, 7];
        // First: find max in [1,2,9] -> 9 at index 2
        // Second: find max in [3] (index 3 to 3) -> 3 at index 3
        // Wait, let me recalculate:
        // end = 5 - (3 - 0 - 1) = 5 - 2 = 3, so bank[0..3] = [1,2,9], max=9 at idx 2
        // end = 5 - (3 - 1 - 1) = 5 - 1 = 4, so bank[3..4] = [3], max=3 at idx 3
        // end = 5 - (3 - 2 - 1) = 5 - 0 = 5, so bank[4..5] = [7], max=7 at idx 4
        // Result: 937
        assert_eq!(get_joltage(&bank, 3), 937);
    }

    #[test]
    fn test_get_joltage_all_digits() {
        let bank = vec![1, 2, 3];
        // Must select all 3 digits in order: 1, 2, 3 -> 123
        assert_eq!(get_joltage(&bank, 3), 123);
    }

    #[test]
    fn test_get_joltage_single_digit() {
        let bank = vec![5, 3, 8, 1];
        // Find 1 digit: max of entire bank = 8
        assert_eq!(get_joltage(&bank, 1), 8);
    }

    #[test]
    fn test_get_joltage_descending_order() {
        let bank = vec![9, 8, 7, 6];
        // Find 2 digits from descending sequence
        // First: max in [9,8,7] = 9 at idx 0
        // Second: max in [8,7,6] = 8 at idx 1
        // Result: 98
        assert_eq!(get_joltage(&bank, 2), 98);
    }

    #[test]
    fn test_part_a_simple() {
        let input = vec!["12345".to_string()];
        // Part A finds 2 largest digits in order
        // First: max in [1,2,3,4] = 4 at idx 3
        // Second: max in [5] = 5 at idx 4
        // Result: 45
        assert_eq!(Day3::part_a(&input), "45");
    }

    #[test]
    fn test_part_a_multiple_lines() {
        let input = vec!["123".to_string(), "987".to_string()];
        // Line 1 "123": find 2 digits -> max in [1,2]=2, then max in [3]=3 -> 23
        // Line 2 "987": find 2 digits -> max in [9,8]=9, then max in [8,7]=8 -> 98
        // Sum: 23 + 98 = 121
        assert_eq!(Day3::part_a(&input), "121");
    }

    #[test]
    fn test_part_b_simple() {
        let input = vec!["123456789012".to_string()];
        // Part B finds 12 largest digits in order (all digits)
        // With 12 digits and finding 12, we get the number as-is
        assert_eq!(Day3::part_b(&input), "123456789012");
    }

    #[test]
    fn test_part_b_multiple_lines() {
        let input = vec![
            "111111111111".to_string(),
            "222222222222".to_string(),
        ];
        // Each line produces 111111111111 and 222222222222
        // Sum: 111111111111 + 222222222222 = 333333333333
        assert_eq!(Day3::part_b(&input), "333333333333");
    }

    #[test]
    fn test_part_a_all_same_digits() {
        let input = vec!["5555".to_string()];
        // All 5s, find 2 -> 55
        assert_eq!(Day3::part_a(&input), "55");
    }
}
