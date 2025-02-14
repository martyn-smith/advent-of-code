use crate::intcode;
use crate::intcode::{ascii, FromStr, Program};
use itertools::iproduct;
use ndarray::Array;

pub fn get_input() -> Program {
    Program::from_str(include_str!("../../data/17.txt")).unwrap()
}

pub fn part_1(program: &Program) -> usize {
    let program = program.clone();
    let _pic = intcode::ascii(program, None).unwrap();
    let pic = include_str!("../../data/pic.txt");
    let width = pic.lines().next().unwrap().len();
    let depth = pic.lines().count();
    let pic = pic
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c == '#')
        .collect::<Vec<bool>>();
    let arr = Array::from_shape_vec((depth, width), pic).unwrap();
    let intersections = iproduct!(1..depth - 1, 1..width - 1)
        .filter(|&(i, j)| {
            {
                let m = arr[[i, j]]
                    && arr[[i - 1, j]]
                    && arr[[i + 1, j]]
                    && arr[[i, j - 1]]
                    && arr[[i, j + 1]];
                if m { /*println!("match!")*/ }
                m
            }
        })
        .map(|(i, j)| i * j)
        .collect::<Vec<usize>>();
    intersections.iter().sum()
}
