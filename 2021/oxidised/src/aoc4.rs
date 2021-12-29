///
/// Advent of Code day 4: Squid Bingo!
///

use ndarray::{Array, Array2};

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
    board: Array2<Option<usize>>,
    picked: Vec<usize>
}

impl Board {
    fn new(b: &str) -> Self {
        let n = b.split_whitespace()
                 .map(|c| c.parse::<usize>().ok())
                 .collect();
        Self {
            board: Array::from_shape_vec((BOARD_SIZE, BOARD_SIZE), n).unwrap(),
            picked: vec![]
        }
    }

    fn play(&mut self, n: &usize) -> Option<usize> {
        //clippy appears wrong to require flatten here - we need to access point as an enum
        #[allow(clippy::manual_flatten)]
        for point in self.board.iter_mut() {
            if let Some(c) = point {
                if c == n {
                    *point = None;
                    self.picked.push(*n);
                }
            }
        }
        for r in self.board.rows() {
            if r.iter().all(|p| p.is_none()) {
                return Some(self.score(n));
            }
        }
        for c in self.board.columns() {
            if c.iter().all(|p| p.is_none()) {
                return Some(self.score(n));
            }
        }
        None
    }

    fn score(&self, n: &usize) -> usize {
        self.board.iter().filter_map(|&p| p).sum::<usize>() * n
    }
}

pub fn get_input() -> (Vec<usize>, Vec<Board>) {
    let mut groups = include_str!("../../data/4.txt").split("\n\n");

    let cmd = groups.next().unwrap()
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect();

    let boards = groups
                    .map(|b|
                        {
                            Board::new(b)
                        })
                    .collect();

    (cmd, boards)
}

pub fn part_1(input: &(Vec<usize>, Vec<Board>)) -> usize {
    let mut cmd = input.0.iter();
    let mut boards = input.1.clone();
    loop {
        let n = cmd.next().unwrap();
        for board in boards.iter_mut() {
            if let Some(p) = board.play(n) {
                return p;
            }
        }
    }
}

pub fn part_2(input: &(Vec<usize>, Vec<Board>)) -> usize {
    let mut cmd = input.0.iter();
    let mut boards = input.1.clone();
    let mut score = 0;
    while !boards.is_empty() {
        let n = cmd.next().unwrap();
        let mut retain = vec![true; boards.len()];

        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(p) = board.play(n) {
                score = p;
                retain[i] = false;
            }
        }
        let mut retain = retain.iter();
        boards.retain(|_| *retain.next().unwrap());
    }
    score
}
