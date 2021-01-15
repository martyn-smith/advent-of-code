use itertools::Itertools;
use std::fs;

enum Direction {
    Horizontal,
    Vertical,
}

struct Intersection {
    r: usize,
    t: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    l: usize,
}

impl Point {
    fn new(x: i32, y: i32, l: usize) -> Self {
        Point {
            x,
            y,
            l: l + x.abs() as usize + y.abs() as usize,
        }
    }
}

impl Intersection {
    fn new(p: Point, p1: &Point, p2: &Point) -> Self {
        Intersection {
            r: p.x.abs() as usize + p.y.abs() as usize,
            t: p.l,
        }
    }
}

fn direction(p: &(Point, Point)) -> Direction {
    if p.0.x == p.1.x {
        return Direction::Vertical;
    } else if p.0.y == p.1.y {
        return Direction::Horizontal;
    } else {
        panic!("{:?} {:?}", p.0, p.1);
    }
}

fn is_intersection(p1: &(Point, Point), p2: &(Point, Point)) -> Option<Intersection> {
    match (direction(&p1), direction(&p2)) {
        (Direction::Horizontal, Direction::Vertical) => {
            if ((p1.0.x < p2.0.x && p2.0.x < p1.1.x) || (p1.1.x < p2.0.x && p2.0.x < p1.0.x))
                && ((p2.0.y < p1.0.y && p1.0.y < p2.1.y) || (p2.1.y < p1.0.y && p1.0.y < p2.0.y))
            {
                Some(Intersection::new(Point::new(
                    p2.0.x,
                    p1.0.y,
                    p1.0.l + p2.0.l,
                ), &p1.0, &p2.0))
            } else {
                None
            }
        }
        (Direction::Vertical, Direction::Horizontal) => {
            if ((p1.0.y < p2.0.y && p2.0.y < p1.1.y) || (p1.1.y < p2.0.y && p2.0.y < p1.0.y))
                && ((p2.0.x < p1.0.x && p1.0.x < p2.1.x) || (p2.1.x < p1.0.x && p1.0.x < p2.0.x))
            {
                Some(Intersection::new(Point::new(
                    p1.0.x,
                    p2.0.y,
                    p1.0.l + p2.0.l,
                ), &p1.0, &p2.0))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_wire(line: &str) -> Vec<Point> {
    let mut wire: Vec<Point> = vec![Point::new(0, 0, 0)];
    for l in line.trim().split(',') {
        let mag = str::parse::<i32>(&l[1..]).unwrap();
        let vec = match l.chars().nth(0).unwrap() {
            'L' => (-mag, 0i32),
            'R' => (mag, 0i32),
            'U' => (0i32, mag),
            'D' => (0i32, -mag),
            _ => {
                panic!();
            }
        };
        let last = &wire[wire.len() - 1];
        let pos = Point::new(last.x + vec.0, last.y + vec.1, last.l);
        wire.push(pos);
    }
    wire
}

pub fn get_input() -> (Vec<Point>, Vec<Point>) {
    let input = fs::read_to_string("../data/3.txt").unwrap();
    let wires = input
        .lines()
        .map(|l| get_wire(l))
        .collect::<Vec<Vec<Point>>>();
    (wires[0].clone(), wires[1].clone())
}

pub fn part_1(input: &(Vec<Point>, Vec<Point>)) -> usize {
    input
        .0
        .windows(2)
        .cartesian_product(input.1.windows(2))
        .filter_map(|(trace_1, trace_2)| {
            is_intersection(&(trace_1[0], trace_1[1]), &(trace_2[0], trace_2[1]))
        })
        .map(|p| p.r)
        .min()
        .unwrap()
}

pub fn part_2(input: &(Vec<Point>, Vec<Point>)) -> usize {
    input
        .0
        .windows(2)
        .cartesian_product(input.1.windows(2))
        .filter_map(|(trace_1, trace_2)| {
            is_intersection(&(trace_1[0], trace_1[1]), &(trace_2[0], trace_2[1]))
        })
        .map(|p| p.t)
        .min()
        .unwrap()
}
