///
/// Advent of Code day 15: Dijikstra / A* time
///

//use itertools::iproduct;
use ndarray::{Array, Array2, Axis};
use std::collections::BinaryHeap;
use std::collections::LinkedList;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    pos: [usize; 2],
    cost: usize,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos[0].cmp(&other.pos[0]))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_candidates(pos: &[usize; 2], d: usize, w: usize) -> Vec<[usize; 2]> {
    let mut candidates = vec![];
    if pos[0] < d {
        candidates.push([pos[0]+1,pos[1]]);
    }

    if pos[1] < w {
        candidates.push([pos[0],pos[1]+1]);
    }

    if pos[0] > 0 {
        candidates.push([pos[0]-1,pos[1]]);
    }

    if pos[1] > 0 {
        candidates.push([pos[0],pos[1]-1]);
    }
    candidates
}

fn djikstra(risks: &Array2<u8>) -> usize {
    let d = risks.nrows() - 1;
    let w = risks.ncols() - 1;
    let mut pos = Pos{ [0,0], 0};
    let target = [d,w];
    let mut risks = BinaryHeap::new();
    risks.push(pos);
    while let Some(pos) = risks.pop() {
        if pos.pos == target {
            return pos.cost;
        }

        if pos.cost > dist[position] {
            continue;
        }

        for cand in get_candidates(pos.pos, d, w).into_iter() {
            //do stuff
        }

    }

}


pub fn get_input() -> Array2<u8> {
    let cave = include_str!("../../data/15.small.txt");
    let width = cave.lines().next().unwrap().len();
    let depth = cave.lines().count();
    let risks = cave
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let risks = Array::from_shape_vec((depth, width), risks).unwrap();
    risks
}

pub fn part_1(input: &Array2<u8>) -> usize {
    djikstra(input)
}


pub fn part_2(input: &Array2<u8>) -> usize {
    /// risk level is 1 higher than the tile immediately up or to the left of it
    /// i.e.     0 1 2
    ///          1 1 2
    ///          2 2 2
    ///
    /// so, we need to solve for all UL edge points to all DR edge points, and use
    /// those to feed a new djikstra solver.

    let mut cave = input.clone();
    for i in 1..=5 {

    }
    0
}
