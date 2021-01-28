use ndarray::{Array, Array2};
use std::fs;
use std::iter::FromIterator;

fn count_asteroids(point: (usize, usize), asteroid_map: &Array2<bool>) -> usize {0}

pub fn get_input() -> Array2<bool> {
    let input = fs::read_to_string("../data/10.txt").unwrap();
    //Array::from_iter(input.lines().map(|l| l.chars().map(|c| c == '#').collect()))
}

pub fn part_1(input: &Array2<bool>) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(point, _)| count_asteroids(point, &input))
        .max()
        .unwrap()
}
