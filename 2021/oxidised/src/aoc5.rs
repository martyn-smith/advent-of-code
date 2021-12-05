use itertools::{Either, Itertools, iproduct};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal
}

#[derive(Debug, Clone)]
pub struct Line {
    x: (usize, usize),
    y: (usize, usize),
}

impl Line {
    fn new(l: &str) -> Self {
        let mut l = l.split(" -> ");
        let mut start = l.next().unwrap().split(",");
        let mut end = l.next().unwrap().split(",");
        let mut x = (start.next().unwrap().parse::<usize>().unwrap(),
                     end.next().unwrap().parse::<usize>().unwrap());
        let mut y = (start.next().unwrap().parse::<usize>().unwrap(),
                     end.next().unwrap().parse::<usize>().unwrap());
        //simplify by making sure x always increases
        if x.0 > x.1 {
            x = (x.1, x.0);
            y = (y.1, y.0);
        }
        Self {
            x,
            y
        }
    }

    fn direction(&self) -> Direction {
        if self.y.0 == self.y.1 {
            Direction::Horizontal
        } else if self.x.0 == self.x.1 {
            Direction::Vertical
        } else {
            Direction::Diagonal
        }
    }

    fn draw(&self, once: &mut HashSet<(usize, usize)>, twice: &mut HashSet<(usize, usize)>) {
        match self.direction() {
            Direction::Horizontal => {
                //let pts = (self.x.0..=self.x.1).zip(std::iter::repeat(self.y.0));
                let x_rng = self.x.0..=self.x.1;
                let y = self.y.0;
                for x in x_rng {
                    if !once.insert((x, y)) {
                        twice.insert((x, y));
                    }
                }
            },
            Direction::Vertical => {
                //let pts = (self.x.0..=self.x.1).rev().zip(std::iter::repeat(self.y.0));
                let y_rng = if self.y.1 < self.y.0 {self.y.1..=self.y.0} else {self.y.0..=self.y.1};
                let x = self.x.0;
                for y in y_rng {
                    if !once.insert((x, y)) {
                        twice.insert((x, y));
                    }
                }
            },
            Direction::Diagonal => {
                let rev = self.y.1 < self.y.0;
                let x_rng = (self.x.0..=self.x.1);
                let y_rng = if rev {
                    Either::Right((self.y.1..=self.y.0).rev())
                } else {
                    Either::Left(self.y.0..=self.y.1)
                };
                for (x,y) in x_rng.zip(y_rng) {
                    if !once.insert((x, y)) {
                        twice.insert((x, y));
                    }
                }
            }
        };

        // for (x, y) in pts {
        //     if !once.insert((x,y)) {
        //         println!("({}, {}) exist", x, y);
        //         twice.insert((x,y));
        //     }
        // }

    }

}

pub fn get_input() -> Vec<Line> {
    include_str!("../../data/5.txt")
        .lines()
        .map(|l| Line::new(l))
        .collect()
}

pub fn part_1(input: &Vec<Line>) -> usize {
    let (mut once, mut twice) = (HashSet::new(), HashSet::new());
    let lines = input.iter().filter(|&l| l.direction() != Direction::Diagonal);
    for l in lines {
        l.draw(&mut once, &mut twice);
    }
    twice.len()
}

pub fn part_2(input: &Vec<Line>) -> usize {
    let (mut once, mut twice) = (HashSet::new(), HashSet::new());
    let lines = input.iter();
    for l in lines {
        l.draw(&mut once, &mut twice);
    }
    twice.len()
}
