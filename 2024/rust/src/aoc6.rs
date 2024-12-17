use ndarray::{Array, Array2};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Guard {
    pos: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new(input: &str) -> Self {
        let (pos, direction) = input
            .lines()
            .enumerate()
            .filter_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(j, r)| match r {
                        '^' => Some(((i, j), Direction::Up)),
                        '>' => Some(((i, j), Direction::Right)),
                        'v' => Some(((i, j), Direction::Down)),
                        '<' => Some(((i, j), Direction::Left)),
                        _ => None,
                    })
                    .next()
            })
            .next()
            .unwrap();
        Self { pos, direction }
    }

    fn walk(&mut self, maze: &Array2<bool>) -> usize {
        let mut visited = HashSet::<(usize, usize)>::new();
        while (self.pos.0 > 0 && self.pos.0 < maze.nrows() - 1)
            && (self.pos.1 > 0 && self.pos.1 < maze.ncols() - 1)
        {
            visited.insert(self.pos);
            match self.direction {
                Direction::Up => {
                    if maze[[self.pos.0 - 1, self.pos.1]] {
                        self.direction = Direction::Right;
                    } else {
                        self.pos.0 -= 1;
                    }
                }
                Direction::Down => {
                    if maze[[self.pos.0 + 1, self.pos.1]] {
                        self.direction = Direction::Left;
                    } else {
                        self.pos.0 += 1;
                    }
                }
                Direction::Left => {
                    if maze[[self.pos.0, self.pos.1 - 1]] {
                        self.direction = Direction::Up;
                    } else {
                        self.pos.1 -= 1;
                    }
                }
                Direction::Right => {
                    if maze[[self.pos.0, self.pos.1 + 1]] {
                        self.direction = Direction::Down;
                    } else {
                        self.pos.1 += 1;
                    }
                }
            }
        }
        visited.insert(self.pos);
        visited.len()
    }

    fn walk_2(&mut self, maze: &Array2<bool>) -> usize {
        let mut visited = HashSet::<(usize, usize)>::new();
        while (self.pos.0 > 0 && self.pos.0 < maze.nrows() - 1)
            && (self.pos.1 > 0 && self.pos.1 < maze.ncols() - 1)
        {
            visited.insert(self.pos);
            match self.direction {
                Direction::Up => {
                    if maze[[self.pos.0 - 1, self.pos.1]] {
                        self.direction = Direction::Right;
                    } else {
                        self.pos.0 -= 1;
                    }
                }
                Direction::Down => {
                    if maze[[self.pos.0 + 1, self.pos.1]] {
                        self.direction = Direction::Left;
                    } else {
                        self.pos.0 += 1;
                    }
                }
                Direction::Left => {
                    if maze[[self.pos.0, self.pos.1 - 1]] {
                        self.direction = Direction::Up;
                    } else {
                        self.pos.1 -= 1;
                    }
                }
                Direction::Right => {
                    if maze[[self.pos.0, self.pos.1 + 1]] {
                        self.direction = Direction::Down;
                    } else {
                        self.pos.1 += 1;
                    }
                }
            }
        }
        visited.insert(self.pos);
        0
        //    let cands = visited.into_iter()
        //                .map(|r| [
        //                    (r.0.checked_sub(1), r.1.checked_sub(1)),
        //                    (r.0.checked_sub(1), r.1),
        //                    (r.0.checked_sub(1), r.1 + 1),
        //                    (r.0, r.1.checked_sub(1)),
        //                    (r.0, r.1 + 1),
        //                    (r.0 + 1, r.1.checked_sub(1)),
        //                    (r.0 + 1, r.1),
        //                    (r.0 + 1, r.1 + 1)
        //                ].into_iter()
        //                 .collect::<Vec<_>>()
        //                    )
        //                .flatten();
        //    cands.into_iter()
        //        .filter(|c| {
        //        maze[[c.0, c.1]] = true;
        //    let mut visited = HashSet::<(usize, usize)>::new();
        //    while (self.pos.0 > 0 && self.pos.0 < maze.nrows() - 1)
        //        && (self.pos.1 > 0 && self.pos.1 < maze.ncols() - 1) {
        //            if visited.contains(self.pos) {
        //                return false;
        //            } else {
        //            match self.direction {
        //                Direction::Up => {
        //                    if maze[[self.pos.0 - 1, self.pos.1]] {
        //                        self.direction = Direction::Right;
        //                    } else {
        //                        self.pos.0 -= 1;
        //                    }
        //                },
        //                Direction::Down => {
        //                    if maze[[self.pos.0 + 1, self.pos.1]] {
        //                        self.direction = Direction::Left;
        //                    } else {
        //                        self.pos.0 += 1;
        //                    }
        //                },
        //                Direction::Left => {
        //                    if maze[[self.pos.0, self.pos.1 - 1]] {
        //                        self.direction = Direction::Up;
        //                    } else {
        //                        self.pos.1 -= 1;
        //                    }
        //                },
        //                Direction::Right => {
        //                    if maze[[self.pos.0, self.pos.1 + 1]] {
        //                        self.direction = Direction::Down;
        //                    } else {
        //                        self.pos.1 += 1;
        //                    }
        //                }
        //            }
        //            true
        //        }
        //        }
        //
        //    }
        //
    }
}

pub fn get_input() -> (Array2<bool>, Guard) {
    let input = include_str!("../../data/6.txt");
    let width = input.lines().next().unwrap().len();
    let depth = input.lines().count();
    let m = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c == '#')
        .collect::<Vec<bool>>();
    let maze = Array::from_shape_vec((width, depth), m).unwrap();
    let guard = Guard::new(input);
    (maze, guard)
}

pub fn part_1(input: &(Array2<bool>, Guard)) -> usize {
    let mut guard = input.1.clone();
    guard.walk(&input.0)
}

pub fn part_2(input: &(Array2<bool>, Guard)) -> usize {
    let mut guard = input.1.clone();
    guard.walk_2(&input.0)
}
