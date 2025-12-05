use crate::day::Day;

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

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
enum Side {
    Start,
    End,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
struct Boundary {
    value: u64,
    side: Side,
}
