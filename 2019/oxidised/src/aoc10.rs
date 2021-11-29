use ndarray::{Array, Array2};
use std::fs;
use std::iter::FromIterator;

fn count_asteroids(point: (usize, usize), asteroid_map: &Array2<bool>) -> usize {
    0
}

pub fn get_input() -> Array2<bool> {
    let input = fs::read_to_string("../data/10.txt").unwrap();
    let width = input.lines().next().unwrap().len();
    let depth = input.lines().count();
    let input = input.chars()
                     .filter(|&c| c != '\n')
                     .map(|c| c == '#')
                     .collect::<Vec<bool>>();
    println!("{}", input.len());
    Array::from_shape_vec(
        (width, depth), input
        ).unwrap()
}

pub fn part_1(input: &Array2<bool>) -> usize {
    println!("{:?}", input);
    (0..input.nrows())
        .zip(0..input.ncols())
        .filter(|(i, j)| *input.get([*i, *j]).unwrap())
        .map(|point| count_asteroids(point, &input))
        .max()
        .unwrap()
}
