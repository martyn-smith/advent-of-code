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
        Point { x, y, l }
    }
}

impl Intersection {
    fn new(p: Point, p1: &Point, p2: &Point) -> Self {
        //println!("{:?} {} {}", p, p1.l, p2.l);
        Intersection {
            r: p.x.abs() as usize + p.y.abs() as usize,
            t: p1.l
                + p2.l
                + (p.x - p1.x).abs() as usize
                + (p.y - p1.y).abs() as usize
                + (p.x - p2.x).abs() as usize
                + (p.y - p2.y).abs() as usize,
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

fn get_intersection(a: &(Point, Point), b: &(Point, Point)) -> Option<Intersection> {
    match (direction(&a), direction(&b)) {
        (Direction::Horizontal, Direction::Vertical) => {
            if ((a.0.x < b.0.x && b.0.x < a.1.x) || (a.1.x < b.0.x && b.0.x < a.0.x))
                && ((b.0.y < a.0.y && a.0.y < b.1.y) || (b.1.y < a.0.y && a.0.y < b.0.y))
            {
                Some(Intersection::new(
                    Point::new(b.0.x, a.0.y, a.0.l + b.0.l),
                    &a.0,
                    &b.0,
                ))
            } else {
                None
            }
        }
        (Direction::Vertical, Direction::Horizontal) => {
            if ((a.0.y < b.0.y && b.0.y < a.1.y) || (a.1.y < b.0.y && b.0.y < a.0.y))
                && ((b.0.x < a.0.x && a.0.x < b.1.x) || (b.1.x < a.0.x && a.0.x < b.0.x))
            {
                Some(Intersection::new(
                    Point::new(a.0.x, b.0.y, a.0.l + b.0.l),
                    &a.0,
                    &b.0,
                ))
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
        let pos = Point::new(last.x + vec.0, last.y + vec.1, last.l + mag as usize);
        wire.push(pos);
    }
    wire
}

pub fn get_input() -> (Vec<Point>, Vec<Point>) {
    let input = include_str!("../../data/3.txt");
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
        .filter_map(|(wire_1, wire_2)| {
            get_intersection(&(wire_1[0], wire_1[1]), &(wire_2[0], wire_2[1]))
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
        .filter_map(|(wire_1, wire_2)| {
            get_intersection(&(wire_1[0], wire_1[1]), &(wire_2[0], wire_2[1]))
        })
        .map(|p| p.t)
        .min()
        .unwrap()
}
