//! Day 8: Point Clustering
//!
//! Clusters 3D points by Euclidean distance using a Kruskal-style algorithm.
//! Points are grouped into circuits by connecting the closest pairs first.
//!
//! ## Input Format
//! Each line contains comma-separated 3D coordinates: `x,y,z`.
//!
//! ## Part A
//! Finds the product of the sizes of the three largest circuits after
//! connecting the closest `pair_limit` pairs.
//!
//! ## Part B
//! Finds the pair of points that, when connected, joins all points into
//! a single circuit. Returns the product of their x-coordinates.

use crate::day::Day;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Solution for Day 8: Point Clustering puzzle.
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

        let mut circuit_sizes: Vec<_> = circuits
            .into_iter()
            .filter(|c| !c.is_empty())
            .map(|c| c.len())
            .collect();
        println!("{:?}", circuit_sizes);

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

/// An N-dimensional point used for clustering.
///
/// Generic over the number of dimensions N (typically 3 for this puzzle).
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ConnectionBox<const N: usize> {
    /// The coordinates in N-dimensional space
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
    /// Creates a new connection box from a vector of coordinates.
    ///
    /// # Panics
    /// Panics if the vector length doesn't match N.
    fn new(coords: Vec<i64>) -> Self {
        if coords.len() != N {
            panic!();
        }

        Self {
            coords: coords.try_into().expect("bad vec size"),
        }
    }

    /// Calculates the integer Euclidean distance to another point.
    ///
    /// Uses integer square root for the final result.
    fn distance(&self, other: &Self) -> i64 {
        let mut total = 0;
        for (s, o) in self.coords.iter().zip(other.coords.iter()) {
            total += (s - o).pow(2);
        }

        total.isqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_box_new() {
        let b: ConnectionBox<3> = ConnectionBox::new(vec![1, 2, 3]);
        assert_eq!(b.coords, [1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_connection_box_new_wrong_size() {
        let _: ConnectionBox<3> = ConnectionBox::new(vec![1, 2]);
    }

    #[test]
    fn test_connection_box_distance_same_point() {
        let b1: ConnectionBox<3> = ConnectionBox::new(vec![0, 0, 0]);
        let b2: ConnectionBox<3> = ConnectionBox::new(vec![0, 0, 0]);
        assert_eq!(b1.distance(&b2), 0);
    }

    #[test]
    fn test_connection_box_distance_simple() {
        let b1: ConnectionBox<3> = ConnectionBox::new(vec![0, 0, 0]);
        let b2: ConnectionBox<3> = ConnectionBox::new(vec![3, 4, 0]);
        // sqrt(3^2 + 4^2 + 0^2) = sqrt(25) = 5
        assert_eq!(b1.distance(&b2), 5);
    }

    #[test]
    fn test_connection_box_distance_3d() {
        let b1: ConnectionBox<3> = ConnectionBox::new(vec![0, 0, 0]);
        let b2: ConnectionBox<3> = ConnectionBox::new(vec![1, 2, 2]);
        // sqrt(1 + 4 + 4) = sqrt(9) = 3
        assert_eq!(b1.distance(&b2), 3);
    }

    #[test]
    fn test_connection_box_hash_eq() {
        use std::collections::HashSet;

        let b1: ConnectionBox<3> = ConnectionBox::new(vec![1, 2, 3]);
        let b2: ConnectionBox<3> = ConnectionBox::new(vec![1, 2, 3]);

        let mut set = HashSet::new();
        set.insert(&b1);
        assert!(set.contains(&b2));
    }

    #[test]
    fn test_part_a_three_points_triangle() {
        let input = vec![
            "0,0,0".to_string(),
            "3,0,0".to_string(),
            "0,4,0".to_string(),
        ];
        // 3 points form a triangle, all close together
        // With pair_limit based on input size, forms 1 circuit of size 3
        // Product of top 3 circuit sizes: 3 * 1 * 1 = 3 (or just 3 if only 1 circuit)
        let result = Day8::part_a(&input);
        // The result depends on the algorithm - let's just verify it runs
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part_a_two_clusters() {
        let input = vec![
            "0,0,0".to_string(),
            "1,0,0".to_string(),
            "100,0,0".to_string(),
            "101,0,0".to_string(),
        ];
        // Two clusters far apart: (0,0,0)-(1,0,0) and (100,0,0)-(101,0,0)
        // Should form 2 circuits of size 2 each
        // Product of top 3: 2 * 2 * 1 = 4
        let result = Day8::part_a(&input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part_b_simple() {
        let input = vec![
            "0,0,0".to_string(),
            "1,0,0".to_string(),
            "2,0,0".to_string(),
        ];
        // Simple linear arrangement
        let result = Day8::part_b(&input);
        assert_eq!(result, "2");
    }
}
