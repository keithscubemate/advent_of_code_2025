use crate::day::Day;
use rayon::prelude::*;

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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: Self) -> u64 {
        let l = (self.x - other.x).abs() + 1;
        let h = (self.y - other.y).abs() + 1;

        (l * h) as u64
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

        Self { p1, p2 }
    }

    fn x_line(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn x_in(&self, x: i64) -> bool {
        let x1 = self.p1.x;
        let x2 = self.p2.x;

        x1 <= x && x <= x2 || x2 <= x && x <= x1
    }

    fn y_line(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn y_in(&self, y: i64) -> bool {
        let y1 = self.p1.y;
        let y2 = self.p2.y;

        y1 <= y && y <= y2 || y2 <= y && y <= y1
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Square {
    nw: Point,
    ne: Point,
    sw: Point,
    se: Point,
}

impl Square {
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

struct Atlas {
    points: Vec<Point>,
    x_lines: Vec<Line>,
    y_lines: Vec<Line>,
}

impl Atlas {
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

    fn is_valid(&self, point: Point) -> bool {
        if self.is_red(point) {
            return true;
        }

        self.is_green(point)
    }

    fn is_red(&self, point: Point) -> bool {
        self.points.iter().any(|p| *p == point)
    }

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
