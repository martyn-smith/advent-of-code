///
/// Advent of Code day 9: Finding Minima
///
use itertools::iproduct;
use ndarray::{Array, Array2};
use std::collections::HashSet;

fn expand(
    pt: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    input: &Array2<usize>,
) -> Vec<(usize, usize)> {
    let mut pts = vec![];
    let (i_max, j_max) = (input.nrows() - 1, input.ncols() - 1);
    let (i, j) = (pt.0, pt.1);
    if input[[i, j]] != 9 {
        visited.insert((i, j));
        pts.push((i, j));
        if i < i_max && !visited.contains(&(i + 1, j)) {
            pts.extend(expand((i + 1, j), visited, input));
        }
        if i > 0 && !visited.contains(&(i - 1, j)) {
            pts.extend(expand((i - 1, j), visited, input));
        }
        if j < j_max && !visited.contains(&(i, j + 1)) {
            pts.extend(expand((i, j + 1), visited, input));
        }
        if j > 0 && !visited.contains(&(i, j - 1)) {
            pts.extend(expand((i, j - 1), visited, input));
        }
    }
    pts
}

fn find_low_points(input: &Array2<usize>) -> Vec<(usize, usize)> {
    let mut pts = vec![];
    let i_max = input.nrows() - 1;
    let j_max = input.ncols() - 1;
    for (i, j) in iproduct!(1..i_max, 1..j_max).filter(|&(i, j)| {
        let m = input[[i, j]];
        m < input[[i - 1, j]]
            && m < input[[i + 1, j]]
            && m < input[[i, j - 1]]
            && m < input[[i, j + 1]]
    }) {
        pts.push((i, j));
    }

    //top row
    for i in (1..i_max).filter(|&i| {
        let j = 0;
        let m = input[[i, j]];
        m < input[[i - 1, j]] && m < input[[i + 1, j]] && m < input[[i, j + 1]]
    }) {
        pts.push((i, 0));
    }

    //bottom row
    for i in (1..i_max).filter(|&i| {
        let j = j_max;
        let m = input[[i, j]];
        m < input[[i - 1, j]] && m < input[[i + 1, j]] && m < input[[i, j - 1]]
    }) {
        pts.push((i, j_max));
    }

    //LH column
    for j in (1..j_max).filter(|&j| {
        let i = 0;
        let m = input[[i, j]];
        m < input[[i, j - 1]] && m < input[[i, j + 1]] && m < input[[i + 1, j]]
    }) {
        pts.push((0, j));
    }

    //RH column
    for j in (1..j_max).filter(|&j| {
        let i = i_max;
        let m = input[[i, j]];
        m < input[[i, j - 1]] && m < input[[i, j + 1]] && m < input[[i - 1, j]]
    }) {
        pts.push((i_max, j));
    }

    //corners
    if input[[0, 0]] < input[[1, 0]] && input[[0, 0]] < input[[0, 1]] {
        pts.push((0, 0));
    }
    if input[[i_max, 0]] < input[[i_max - 1, 0]] && input[[i_max, 0]] < input[[i_max, 1]] {
        pts.push((i_max, 0));
    }
    if input[[0, j_max]] < input[[1, j_max]] && input[[0, j_max]] < input[[0, j_max - 1]] {
        pts.push((0, j_max));
    }
    if input[[i_max, j_max]] < input[[i_max - 1, j_max]]
        && input[[i_max, j_max]] < input[[i_max, j_max - 1]]
    {
        pts.push((i_max, j_max));
    }
    pts
}

pub fn get_input() -> Array2<usize> {
    let hghtmap = include_str!("../../data/9.txt");
    let width = hghtmap.lines().next().unwrap().len();
    let depth = hghtmap.lines().count();
    let pic = hghtmap
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    Array::from_shape_vec((depth, width), pic).unwrap()
}

pub fn part_1(input: &Array2<usize>) -> usize {
    let pts = find_low_points(input);
    pts.iter().map(|(i, j)| input[[*i, *j]] + 1).sum()
}

pub fn part_2(input: &Array2<usize>) -> usize {
    let pts = find_low_points(input);
    let mut visited = HashSet::new();
    let mut basins = pts
        .iter()
        .map(|pt| expand(*pt, &mut visited, input))
        .collect::<Vec<Vec<(usize, usize)>>>();
    basins.sort_by_key(|i| i.len());
    basins.iter().rev().take(3).map(|l| l.len()).product()
}
