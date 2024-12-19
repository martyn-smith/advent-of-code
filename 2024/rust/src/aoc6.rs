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

enum WalkResult {
    OutOfBounds(HashSet::<Guard>),
    CycleDetected(HashSet::<Guard>)
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

    fn walk(&mut self, maze: &Array2<bool>) -> WalkResult {
        let mut visited = HashSet::<Guard>::new();
        loop {
            if (self.pos.0 == 0 || self.pos.0 == maze.nrows() - 1)
                || (self.pos.1 == 0 || self.pos.1 == maze.ncols() - 1)
            {
                visited.insert(self.clone());
                return WalkResult::OutOfBounds(visited);
            } else if visited.contains(&self) {
                //dbg!(&visited);
                return WalkResult::CycleDetected(visited);
            } else {
                visited.insert(self.clone());
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
        }
    }

}

fn get_blockers(maze: &Array2<bool>, guard: &Guard, visited: HashSet<Guard>) -> HashSet<Guard> {
    let mut cands = visited.clone();
    cands.remove(&guard);
    cands.into_iter()
        .filter_map(|c| {
            let mut m = maze.clone();
            let mut g = guard.clone();
            m[[c.pos.0, c.pos.1]] = true;
            match g.walk(&m) {
                WalkResult::OutOfBounds(_) => None,
                WalkResult::CycleDetected(_) => Some(c)
            }
        })
        .collect::<HashSet<_>>()
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
    if let WalkResult::OutOfBounds(path) = guard.walk(&input.0) {
        path.into_iter()
            .map(|g| g.pos)
            .collect::<HashSet<_>>()
            .len()
    } else {
        panic!()
    }
}

// < 2002
pub fn part_2(input: &(Array2<bool>, Guard)) -> usize {
    let mut guard = input.1.clone();
    if let WalkResult::OutOfBounds(path) = guard.walk(&input.0) {
        let blockers = get_blockers(&input.0, &input.1, path)
            .into_iter()
            .map(|g| g.pos)
            .collect::<HashSet<_>>();
        blockers.len()
    } else {
        panic!()
    }
}
