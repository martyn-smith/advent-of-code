use ndarray::{Array, Array2};
use std::fs;

// Create a table of i × j (with i and j from 1 to 3)

fn add_layers(a: &Array2<usize>, b: &Array2<usize>) -> Array2<usize> {
    let mut out = b.clone();
    out.zip_mut_with(a, |l2, l1| {
        if *l2 == 2 {
            *l2 = *l1;
        }
    });
    out
}


pub fn get_input() -> Vec<Array2<usize>> {
    let width = 25;
    let height = 6;
    let input = include_str!("../../data/8.txt");
    input
        .trim()
        .chars()
        .map(|l| l.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .chunks(width * height)
        .map(|l| Array::from_shape_vec((height, width), l.to_vec()).unwrap())
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

pub fn part_2(input: &Vec<Array2<usize>>) {
    let width = 25;
    let height = 6;
    let output = input
        .iter()
        .rev()
        .fold(Array2::<usize>::zeros(input[0].raw_dim()), |acc, x| {
            add_layers(&acc, x)
        });
}
