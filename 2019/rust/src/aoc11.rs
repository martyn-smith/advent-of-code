use crate::intcode::{Computer, FromStr, Program};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn rotate_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }
}

struct Robot {
    computer: Computer,
    program: Program,
    position: (isize, isize),
    direction: Direction,
}

impl Robot {
    fn new(program: &Program) -> Self {
        Robot {
            computer: Computer::new(),
            program: program.clone(),
            position: (0, 0),
            direction: Direction::North,
        }
    }

    fn step(&mut self, hull: &mut HashMap<(isize, isize), bool>) -> Option<bool> {
        let mut input = vec![match hull.get(&self.position) {
            Some(&b) => {
                if b {
                    1isize
                } else {
                    0
                }
            }
            None => 0,
        }];
        let output = (
            self.computer
                .next(&mut self.program, Some(&mut input))
                .unwrap()?,
            self.computer.next(&mut self.program, None).unwrap()?,
        );
        let painted = match output.0 {
            0 => hull.insert(self.position, false).is_none(),
            1 => hull.insert(self.position, true).is_none(),
            _ => panic!(),
        };
        match output.1 {
            0 => {
                self.direction.rotate_left();
            }
            1 => {
                self.direction.rotate_right();
            }
            _ => {
                panic!();
            }
        };
        match self.direction {
            Direction::North => {
                self.position.1 += 1;
            }
            Direction::East => {
                self.position.0 += 1;
            }
            Direction::South => {
                self.position.1 -= 1;
            }
            Direction::West => {
                self.position.0 -= 1;
            }
        };
        Some(painted)
    }
}

pub fn get_input() -> Program {
    let program = include_str!("../../data/11.txt");
    Program::from_str(program).unwrap()
}

pub fn part_1(program: &Program) -> usize {
    let mut robot = Robot::new(program);
    let mut hull = HashMap::new();
    while robot.step(&mut hull).is_some() {}
    hull.len()
}

pub fn part_2(program: &Program) -> usize {
    let mut robot = Robot::new(program);
    let mut hull = HashMap::new();
    hull.insert((0, 0), true);
    while robot.step(&mut hull).is_some() {}
    println!("{:?}", hull);
    0
}
