use std::fs;
use ndarray::{Array,Array2};

// Create a table of i × j (with i and j from 1 to 3)

pub fn get_input() -> Vec<Array2<usize>> {
    let width = 25;
    let height = 6;
    let input = fs::read_to_string("../data/8.txt").unwrap();
    input
        .trim()
        .chars()
        .map(|l| l.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .chunks(width * height)
        .map(|l| Array::from_shape_vec((width, height), l.to_vec()).unwrap())
        .collect()
}

pub fn part_1(input: &Vec<Array2<usize>>) -> usize {
    let mut input = input.clone();
    input.sort_by(|a, b| {
        a.iter()
            .filter(|&&i| i == 0)
            .count()
            .cmp(&b.iter().filter(|&&i| i == 0).count())
    });
    input[0].iter().filter(|&&i| i == 1).count() * input[0].iter().filter(|&&i| i == 2).count()
}

//0 is black, 1 is white, and 2 is transparent. So if upper layer is 2, take lower layer

pub fn part_2(input: &Vec<Array2<usize>>) -> usize {
    let width = 25;
    let height = 6;
    0
}
