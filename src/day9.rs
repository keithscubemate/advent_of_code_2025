//! Day 9: Maximum Rectangle
//!
//! Finds the maximum area axis-aligned rectangle from a set of 2D points.
//! Uses parallel processing with rayon for efficient computation.
//!
//! ## Input Format
//! Each line contains comma-separated 2D coordinates: `x,y`.
//!
//! ## Part A
//! Finds the maximum rectangle area formed by any two points as opposite
//! corners, considering all point pairs.
//!
//! ## Part B
//! Finds the maximum rectangle area where the entire perimeter lies within
//! a valid boundary defined by the input points (polygon interior check).

use crate::day::Day;
use rayon::prelude::*;

/// Solution for Day 9: Maximum Rectangle puzzle.
pub struct Day9 {}

impl Day for Day9 {
    fn part_a(lines: &[String]) -> String {
        let points: Vec<_> = lines
            .into_iter()
            .map(|s| {
                let mut nums = s.split(',').map(|n| n.parse::<i64>().unwrap());
                Point {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                }
            })
            .collect();

        points
            .iter()
            .enumerate()
            .flat_map(|(i, p)| (&points[i..]).iter().map(|op| p.area(*op)))
            .filter(|a| *a > 0)
            .max()
            .unwrap()
            .to_string()
    }

    fn part_b(lines: &[String]) -> String {
        let points: Vec<_> = lines
            .into_iter()
            .map(|s| {
                let mut nums = s.split(',').map(|n| n.parse::<i64>().unwrap());
                Point {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                }
            })
            .collect();

        let world = Atlas::new(points.clone());

        let mut max_area = 0;
        for i in 0..points.len() - 1 {
            let point = points[i];
            let points = &points[i + 1..];

            let new_area = points
                .par_iter()
                .map(|other| (other, point.area(*other)))
                .filter(|(_, a)| max_area < *a)
                .filter(|(other, _)| {
                    let square = Square::new(&point, other);

                    let mut perim = square.perimeter();

                    perim.all(|p| world.is_valid(p))
                })
                .map(|(_, a)| a)
                .max();

            if let Some(new_area) = new_area {
                max_area = new_area;
            }
            println!("heck yeah: {} -> {}/{}", max_area, i, points.len());
        }

        max_area.to_string()
    }
}

/// A 2D point with integer coordinates.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Calculates the area of the rectangle formed with another point as opposite corner.
    ///
    /// The area includes the boundary points (hence +1 for each dimension).
    fn area(&self, other: Self) -> u64 {
        let l = (self.x - other.x).abs() + 1;
        let h = (self.y - other.y).abs() + 1;

        (l * h) as u64
    }
}

/// A line segment between two points.
///
/// Points are ordered so p1 < p2 for consistent comparison.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    /// Creates a new line segment, ordering points consistently.
    fn new(p1: Point, p2: Point) -> Self {
        let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

        Self { p1, p2 }
    }

    /// Returns true if this is a vertical line (constant x).
    fn x_line(&self) -> bool {
        self.p1.x == self.p2.x
    }

    /// Checks if an x-coordinate is within this line's x-range.
    fn x_in(&self, x: i64) -> bool {
        let x1 = self.p1.x;
        let x2 = self.p2.x;

        x1 <= x && x <= x2 || x2 <= x && x <= x1
    }

    /// Returns true if this is a horizontal line (constant y).
    fn y_line(&self) -> bool {
        self.p1.y == self.p2.y
    }

    /// Checks if a y-coordinate is within this line's y-range.
    fn y_in(&self, y: i64) -> bool {
        let y1 = self.p1.y;
        let y2 = self.p2.y;

        y1 <= y && y <= y2 || y2 <= y && y <= y1
    }
}

/// An axis-aligned rectangle defined by its four corners.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Square {
    nw: Point,
    ne: Point,
    sw: Point,
    se: Point,
}

impl Square {
    /// Creates a rectangle from two opposite corner points.
    fn new(p1: &Point, p2: &Point) -> Self {
        let (x1, x2) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };

        let (y1, y2) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };

        Self {
            nw: Point { x: x1, y: y1 },
            ne: Point { x: x2, y: y1 },
            sw: Point { x: x1, y: y2 },
            se: Point { x: x2, y: y2 },
        }
    }

    /// Returns an iterator over all points on the rectangle's perimeter.
    fn perimeter(&self) -> impl Iterator<Item = Point> {
        let Point { x: x1, y: y1 } = self.nw;
        let Point { x: x2, y: y2 } = self.se;

        let top = (x1..=x2).map(move |x| Point { x, y: y1 });
        let rig = (y1..=y2).map(move |y| Point { x: x1, y });
        let bot = (x1..=x2).map(move |x| Point { x, y: y2 });
        let lef = (y1..=y2).map(move |y| Point { x: x2, y });

        top.chain(rig).chain(bot).chain(lef)
    }
}

/// A spatial index for efficiently checking point validity within a polygon.
///
/// Stores the polygon boundary as separate horizontal and vertical line segments
/// for efficient ray-casting point-in-polygon tests.
struct Atlas {
    /// Original polygon vertices
    points: Vec<Point>,
    /// Vertical line segments (sorted by x)
    x_lines: Vec<Line>,
    /// Horizontal line segments (sorted by y)
    y_lines: Vec<Line>,
}

impl Atlas {
    /// Creates an Atlas from a sequence of polygon vertices.
    ///
    /// Automatically connects the last point to the first to close the polygon.
    fn new(points: Vec<Point>) -> Self {
        let mut lines = vec![];

        for ps in (&points[..]).windows(2) {
            let p1 = ps[0];
            let p2 = ps[1];

            lines.push(Line::new(p1, p2));
        }

        let wrap_line = Line::new(*points.last().unwrap(), *points.first().unwrap());

        lines.push(wrap_line);

        let mut x_lines = vec![];
        let mut y_lines = vec![];

        for l in lines {
            match (l.x_line(), l.y_line()) {
                (true, false) => x_lines.push(l),
                (false, true) => y_lines.push(l),
                _ => panic!(),
            }
        }

        x_lines.sort_by_key(|l| l.p1.x);
        y_lines.sort_by_key(|l| l.p1.y);

        Self {
            points,
            x_lines,
            y_lines,
        }
    }

    /// Finds the maximum rectangle area with all perimeter points valid.
    #[allow(dead_code)]
    fn max_area(self) -> u64 {
        (&self.points[..self.points.len() - 1])
            .par_iter()
            .enumerate()
            .flat_map_iter(|(i, start)| {
                let ends = &self.points[i + 1..];

                ends.into_iter().map(move |e| (start, e))
            })
            .map(|(start, end)| (start, end, start.area(*end)))
            .filter(|(s, e, _)| {
                let square = Square::new(&s, e);

                let mut perim = square.perimeter();

                perim.all(|p| self.is_valid(p))
            })
            .map(|(_, _, a)| a)
            .max()
            .unwrap()
    }

    /// Checks if a point is valid (on boundary or inside polygon).
    fn is_valid(&self, point: Point) -> bool {
        if self.is_red(point) {
            return true;
        }

        self.is_green(point)
    }

    /// Checks if a point is exactly on a polygon vertex.
    fn is_red(&self, point: Point) -> bool {
        self.points.iter().any(|p| *p == point)
    }

    /// Checks if a point is on a polygon edge or inside the polygon.
    fn is_green(&self, point: Point) -> bool {
        let on_bound = self
            .x_lines
            .iter()
            .chain(self.y_lines.iter())
            .any(|l| l.x_in(point.x) && l.y_in(point.y));

        if on_bound {
            return true;
        }

        let x = self.is_in_x_bounds(point);
        let y = self.is_in_y_bounds(point);

        x && y
    }

    /// Checks if a point is inside the polygon using x-axis ray casting.
    fn is_in_x_bounds(&self, point: Point) -> bool {
        let x_lines: Vec<&Line> = self.x_lines.iter().filter(|l| l.y_in(point.y)).collect();

        let mut cross = 0;
        for i in 0..x_lines.len() {
            let l = x_lines[i];
            if i != 0 {
                let l2 = x_lines[i - 1];

                if l.p2.y == l2.p1.y || l.p1.y == l2.p2.y {
                    continue;
                }
            }

            if l.p1.x == point.x {
                return true;
            }

            if l.p1.x > point.x {
                break;
            }

            cross += 1;
        }

        (cross & 1) == 1
    }

    /// Checks if a point is inside the polygon using y-axis ray casting.
    fn is_in_y_bounds(&self, point: Point) -> bool {
        let y_lines: Vec<&Line> = self.y_lines.iter().filter(|l| l.x_in(point.x)).collect();

        let mut cross = 0;
        for i in 0..y_lines.len() {
            let l = y_lines[i];
            if i != 0 {
                let l2 = y_lines[i - 1];

                if l.p2.x == l2.p1.x || l.p1.x == l2.p2.x {
                    continue;
                }
            }

            if l.p1.y == point.y {
                return true;
            }

            if l.p1.y > point.y {
                break;
            }

            cross += 1;
        }

        (cross & 1) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_area_same_point() {
        let p = Point { x: 5, y: 5 };
        // Area of a 1x1 rectangle (same point)
        assert_eq!(p.area(p), 1);
    }

    #[test]
    fn test_point_area_horizontal_line() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 4, y: 0 };
        // Width 5 (0 to 4 inclusive), height 1 = 5
        assert_eq!(p1.area(p2), 5);
    }

    #[test]
    fn test_point_area_vertical_line() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 4 };
        // Width 1, height 5 = 5
        assert_eq!(p1.area(p2), 5);
    }

    #[test]
    fn test_point_area_rectangle() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 2 };
        // Width 4, height 3 = 12
        assert_eq!(p1.area(p2), 12);
    }

    #[test]
    fn test_line_new_orders_points() {
        let p1 = Point { x: 5, y: 5 };
        let p2 = Point { x: 1, y: 1 };
        let line = Line::new(p1, p2);
        // Should order p2 before p1
        assert_eq!(line.p1, p2);
        assert_eq!(line.p2, p1);
    }

    #[test]
    fn test_line_x_line() {
        let vertical = Line::new(Point { x: 5, y: 0 }, Point { x: 5, y: 10 });
        let horizontal = Line::new(Point { x: 0, y: 5 }, Point { x: 10, y: 5 });
        assert!(vertical.x_line());
        assert!(!horizontal.x_line());
    }

    #[test]
    fn test_line_y_line() {
        let vertical = Line::new(Point { x: 5, y: 0 }, Point { x: 5, y: 10 });
        let horizontal = Line::new(Point { x: 0, y: 5 }, Point { x: 10, y: 5 });
        assert!(!vertical.y_line());
        assert!(horizontal.y_line());
    }

    #[test]
    fn test_line_x_in() {
        let line = Line::new(Point { x: 2, y: 0 }, Point { x: 8, y: 0 });
        assert!(line.x_in(2));
        assert!(line.x_in(5));
        assert!(line.x_in(8));
        assert!(!line.x_in(1));
        assert!(!line.x_in(9));
    }

    #[test]
    fn test_line_y_in() {
        let line = Line::new(Point { x: 0, y: 2 }, Point { x: 0, y: 8 });
        assert!(line.y_in(2));
        assert!(line.y_in(5));
        assert!(line.y_in(8));
        assert!(!line.y_in(1));
        assert!(!line.y_in(9));
    }

    #[test]
    fn test_square_new() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: 1, y: 2 };
        let sq = Square::new(&p1, &p2);
        assert_eq!(sq.nw, Point { x: 1, y: 2 });
        assert_eq!(sq.ne, Point { x: 5, y: 2 });
        assert_eq!(sq.sw, Point { x: 1, y: 10 });
        assert_eq!(sq.se, Point { x: 5, y: 10 });
    }

    #[test]
    fn test_square_perimeter() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 2, y: 2 };
        let sq = Square::new(&p1, &p2);
        let perimeter: Vec<Point> = sq.perimeter().collect();
        // Perimeter should include all edge points
        // Top: (0,0), (1,0), (2,0)
        // Right: (0,0), (0,1), (0,2)
        // Bottom: (0,2), (1,2), (2,2)
        // Left: (2,0), (2,1), (2,2)
        assert!(!perimeter.is_empty());
        assert!(perimeter.contains(&Point { x: 0, y: 0 }));
        assert!(perimeter.contains(&Point { x: 2, y: 2 }));
    }

    #[test]
    fn test_part_a_simple() {
        let input = vec![
            "0,0".to_string(),
            "5,0".to_string(),
            "5,5".to_string(),
            "0,5".to_string(),
        ];
        // 4 points forming a square
        // Maximum area rectangle from any two opposite corners
        let result = Day9::part_a(&input);
        assert!(!result.is_empty());
        // The maximum area should be from (0,0) to (5,5) = 6*6 = 36
        assert_eq!(result, "36");
    }

    #[test]
    fn test_part_a_collinear_points() {
        let input = vec![
            "0,0".to_string(),
            "1,0".to_string(),
            "2,0".to_string(),
        ];
        // All points on same line
        // Max area = 3 * 1 = 3 from (0,0) to (2,0)
        let result = Day9::part_a(&input);
        assert_eq!(result, "3");
    }
}
