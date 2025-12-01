use anyhow::Result;

use crate::day::Day;

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

        return format!("{}", zero_count);
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

        return format!("{}", zero_count);
    }
}

fn parse_line(line: &str) -> Result<i32> {
    let negative = line.starts_with('L');

    let val = line[1..].parse::<i32>()?;

    if negative {
        return Ok(val * -1);
    }

    Ok(val)
}
