///
/// Advent of Code day 11: Alarming Nuclear Octopi
///
use itertools::iproduct;
use ndarray::{Array, Array2};

fn flash(input: &mut Array2<u8>) -> usize {
    let (i_max, j_max) = (input.ncols() - 1, input.nrows() - 1);
    //as reminder, ndarrays are column-major
    for (i, j) in iproduct!(0..i_max + 1, 0..j_max + 1) {
        input[[i, j]] += 1;
    }
    let mut reset = vec![];
    loop {
        //body
        let l = reset.len();
        for (i, j) in iproduct!(1..i_max, 1..j_max) {
            if input[[i, j]] > 9 {
                reset.push((i,j));
                input[[i - 1, j - 1]] += 1;
                input[[i - 1, j]] += 1;
                input[[i - 1, j + 1]] += 1;
                input[[i, j - 1]] += 1;
                input[[i, j]] = 0;
                input[[i, j + 1]] += 1;
                input[[i + 1, j - 1]] += 1;
                input[[i + 1, j]] += 1;
                input[[i + 1, j + 1]] += 1;
            }
        }
        //top and bottom rows
        for j in 1..j_max {
            let i = 0;
            if input[[i, j]] > 9 {
                reset.push((i,j));
                input[[i, j - 1]] += 1;
                input[[i, j]] = 0;
                input[[i, j + 1]] += 1;
                input[[i + 1, j - 1]] += 1;
                input[[i + 1, j]] += 1;
                input[[i + 1, j + 1]] += 1;
            }
        }
        for j in 1..j_max {
            let i = i_max;
            if input[[i, j]] > 9 {
                reset.push((i,j));
                input[[i - 1, j - 1]] += 1;
                input[[i - 1, j]] += 1;
                input[[i - 1, j + 1]] += 1;
                input[[i, j - 1]] += 1;
                input[[i, j]] = 0;
                input[[i, j + 1]] += 1;
            }
        }
        //LH and RH col
        for i in 1..i_max {
            let j = 0;
            if input[[i, j]] > 9 {
                reset.push((i,j));
                input[[i - 1, j]] += 1;
                input[[i - 1, j + 1]] += 1;
                input[[i, j]] = 0;
                input[[i, j + 1]] += 1;
                input[[i + 1, j]] += 1;
                input[[i + 1, j + 1]] += 1;
            }
        }
        for i in 1..i_max {
            let j = j_max;
            if input[[i, j]] > 9 {
                reset.push((i,j));
                input[[i - 1, j - 1]] += 1;
                input[[i - 1, j]] += 1;
                input[[i, j - 1]] += 1;
                input[[i, j]] = 0;
                input[[i + 1, j - 1]] += 1;
                input[[i + 1, j]] += 1;
            }
        }
        //corners
        if input[[0, 0]] > 9 {
            reset.push((0,0));
            input[[0, 0]] = 0;
            input[[0, 1]] += 1;
            input[[1, 0]] += 1;
            input[[1, 1]] += 1;
        }
        if input[[0, j_max]] > 9 {
            reset.push((0,j_max));
            input[[0, j_max - 1]] += 1;
            input[[0, j_max]] = 0;
            input[[1, j_max - 1]] += 1;
            input[[1, j_max]] += 1;
        }
        if input[[i_max, 0]] > 9 {
            reset.push((i_max,0));
            input[[i_max - 1, 0]] += 1;
            input[[i_max - 1, 1]] += 1;
            input[[i_max, 0]] = 0;
            input[[i_max, 1]] += 1;
        }
        if input[[i_max, j_max]] > 9 {
            reset.push((i_max,j_max));
            input[[i_max - 1, j_max - 1]] += 1;
            input[[i_max - 1, j_max]] += 1;
            input[[i_max, j_max - 1]] += 1;
            input[[i_max, j_max]] = 0;
        }
        if reset.len() == l {
            break;
        }
    }
    let flashes = reset.len();
    for (i,j) in reset.drain(0..) {
        input[[i,j]] = 0;
    }
    flashes
}

pub fn get_input() -> Array2<u8> {
    let octopi = include_str!("../../data/11.txt");
    let width = octopi.lines().next().unwrap().len();
    let depth = octopi.lines().count();
    let pic = octopi
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let arr = Array::from_shape_vec((depth, width), pic).unwrap();
    arr
}

pub fn part_1(input: &Array2<u8>) -> usize {
    let mut pts = input.clone();
    (0..100u8).map(|_| flash(&mut pts)).sum()
}


pub fn part_2(input: &Array2<u8>) -> usize {
    let mut pts = input.clone();
    let mut rounds = 1;
    while flash(&mut pts) != pts.len() {
        rounds += 1;
    }
    rounds
}
