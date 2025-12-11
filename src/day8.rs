use crate::day::Day;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct Day8 {}

impl Day for Day8 {
    fn part_a(lines: &[String]) -> String {
        let pair_limit = if lines.len() < 100 { 10 } else { 1000 };
        let boxes: Vec<_> = lines
            .into_iter()
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|v| ConnectionBox::<3>::new(v))
            .collect();

        let mut distances = vec![];

        for i in 0..boxes.len() - 1 {
            let b = &boxes[i];
            for ob in &boxes[i + 1..] {
                let dis = b.distance(ob);
                distances.push((dis, b, ob));
            }
        }

        distances.sort();

        let mut circuits: Vec<HashSet<&ConnectionBox<3>>> = vec![];

        for (_, b1, b2) in distances.into_iter().take(pair_limit) {
            if circuits
                .iter()
                .find(|c| c.contains(b1) && c.contains(b2))
                .is_some()
            {
                continue;
            }

            let mut cs = circuits
                .iter_mut()
                .filter(|c| c.contains(b1) || c.contains(b2));

            let c1 = cs.next();
            let c2 = cs.next();

            match (c1, c2) {
                (Some(c1), Some(c2)) => {
                    for b in c2.iter() {
                        c1.insert(b);
                    }
                    c2.clear();
                }
                (Some(c), None) | (None, Some(c)) => {
                    c.insert(b1);
                    c.insert(b2);
                }
                (None, None) => {
                    let mut new_circuit = HashSet::new();
                    new_circuit.insert(b1);
                    new_circuit.insert(b2);
                    circuits.push(new_circuit);
                }
            }
        }

        let mut circuit_sizes: Vec<_> = circuits.into_iter().map(|c| c.len()).collect();

        circuit_sizes.sort();

        circuit_sizes
            .into_iter()
            .rev()
            .take(3)
            .fold(1, |acc, x| acc * x)
            .to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let boxes: Vec<_> = lines
            .into_iter()
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|v| ConnectionBox::<3>::new(v))
            .collect();

        let mut distances = vec![];

        for i in 0..boxes.len() - 1 {
            let b = &boxes[i];
            for ob in &boxes[i + 1..] {
                let dis = b.distance(ob);
                distances.push((dis, b, ob));
            }
        }

        distances.sort();

        let mut circuits: Vec<HashSet<&ConnectionBox<3>>> = vec![];

        let mut bh1 = None;
        let mut bh2 = None;

        for (_, b1, b2) in distances {
            if circuits
                .iter()
                .find(|c| c.contains(b1) && c.contains(b2))
                .is_some()
            {
                continue;
            }

            let mut cs = circuits
                .iter_mut()
                .filter(|c| c.contains(b1) || c.contains(b2));

            let c1 = cs.next();
            let c2 = cs.next();

            match (c1, c2) {
                (Some(c1), Some(c2)) => {
                    for b in c2.iter() {
                        c1.insert(b);
                    }
                    c2.clear();
                }
                (Some(c), None) | (None, Some(c)) => {
                    c.insert(b1);
                    c.insert(b2);
                }
                (None, None) => {
                    let mut new_circuit = HashSet::new();
                    new_circuit.insert(b1);
                    new_circuit.insert(b2);
                    circuits.push(new_circuit);
                }
            }

            let circuit_sizes = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
            let max_circuit_size = circuit_sizes.iter().max().unwrap();

            if circuits.iter().filter(|c| c.len() > 0).count() == 1
                && *max_circuit_size == boxes.len()
            {
                bh1 = Some(b1);
                bh2 = Some(b2);
                break;
            }
        }

        (bh1.unwrap().coords[0] * bh2.unwrap().coords[0]).to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ConnectionBox<const N: usize> {
    coords: [i64; N],
}

impl<const N: usize> Hash for ConnectionBox<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..N {
            self.coords[i].hash(state);
        }
    }
}

impl<const N: usize> ConnectionBox<N> {
    fn new(coords: Vec<i64>) -> Self {
        if coords.len() != N {
            panic!();
        }

        Self {
            coords: coords.try_into().expect("bad vec size"),
        }
    }

    fn distance(&self, other: &Self) -> i64 {
        let mut total = 0;
        for (s, o) in self.coords.iter().zip(other.coords.iter()) {
            total += (s - o).pow(2);
        }

        total.isqrt()
    }
}
