use itertools::iproduct;
///
/// Advent of Code day 18: Game of Life
///
use ndarray::{Array, Array2};
//use std::fs;

#[cfg(debug_assertions)]
fn print(grid: &Array2<bool>) {
    let pic = grid.rows().into_iter().fold("\n".to_string(), |out, r| {
        format!(
            "{}\n{}",
            out,
            r.iter().fold(String::new(), |out, &elem| format!(
                "{}{}",
                out,
                if elem { '#' } else { '.' }
            ))
        )
    });
    println!("{}", pic);
}

fn compute(cell: bool, neighbours: Vec<bool>) -> bool {
    match neighbours.into_iter().filter(|&c| c).count() {
        2 => cell,
        3 => true,
        _ => false,
    }
}

fn step(grid: &mut Array2<bool>) {
    let prev = grid.clone();
    let w = grid.ncols();
    let h = grid.nrows();
    grid[[0, 0]] = compute(grid[[0, 0]], vec![prev[[0, 1]], prev[[1, 0]], prev[[1, 1]]]);
    grid[[w - 1, 0]] = compute(
        grid[[w - 1, 0]],
        vec![prev[[w - 2, 0]], prev[[w - 1, 1]], prev[[w - 2, 1]]],
    );
    grid[[0, h - 1]] = compute(
        grid[[0, h - 1]],
        vec![prev[[0, h - 2]], prev[[1, h - 2]], prev[[1, h - 1]]],
    );
    grid[[w - 1, h - 1]] = compute(
        grid[[w - 1, h - 1]],
        vec![
            prev[[w - 2, h - 2]],
            prev[[w - 2, h - 1]],
            prev[[w - 1, h - 2]],
        ],
    );
    for i in 1..w - 1 {
        grid[[i, 0]] = compute(
            grid[[i, 0]],
            vec![
                prev[[i, 1]],
                prev[[i + 1, 0]],
                prev[[i - 1, 0]],
                prev[[i + 1, 1]],
                prev[[i - 1, 1]],
            ],
        );
        grid[[i, h - 1]] = compute(
            grid[[i, h - 1]],
            vec![
                prev[[i, h - 2]],
                prev[[i + 1, h - 1]],
                prev[[i - 1, h - 1]],
                prev[[i + 1, h - 2]],
                prev[[i - 1, h - 2]],
            ],
        );
    }
    for j in 1..h - 1 {
        grid[[0, j]] = compute(
            grid[[0, j]],
            vec![
                prev[[1, j]],
                prev[[0, j - 1]],
                prev[[0, j + 1]],
                prev[[1, j - 1]],
                prev[[1, j + 1]],
            ],
        );
        grid[[w - 1, j]] = compute(
            grid[[w - 1, j]],
            vec![
                prev[[w - 2, j]],
                prev[[w - 1, j - 1]],
                prev[[w - 1, j + 1]],
                prev[[w - 2, j - 1]],
                prev[[w - 2, j + 1]],
            ],
        );
    }
    for c in iproduct!(1..w - 1, 1..h - 1) {
        grid[[c.0, c.1]] = compute(
            grid[[c.0, c.1]],
            vec![
                prev[[c.0 - 1, c.1 - 1]],
                prev[[c.0 - 1, c.1]],
                prev[[c.0 - 1, c.1 + 1]],
                prev[[c.0, c.1 - 1]],
                prev[[c.0, c.1 + 1]],
                prev[[c.0 + 1, c.1 - 1]],
                prev[[c.0 + 1, c.1]],
                prev[[c.0 + 1, c.1 + 1]],
            ],
        );
    }
}

fn adjust(grid: &mut Array2<bool>) {
    let w = grid.ncols();
    let h = grid.nrows();
    grid[[0, 0]] = true;
    grid[[w - 1, 0]] = true;
    grid[[0, h - 1]] = true;
    grid[[w - 1, h - 1]] = true;
}

pub fn get_input() -> Array2<bool> {
    let input = include_str!("../../data/18.txt");
    let width = input.lines().next().unwrap().len();
    let depth = input.lines().count();
    let input = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c == '#')
        .collect::<Vec<bool>>();
    Array::from_shape_vec((width, depth), input)
        .unwrap()
        .reversed_axes()
}

pub fn part_1(input: &Array2<bool>) -> usize {
    let mut grid = input.clone();
    for _ in 0..100 {
        step(&mut grid);
        #[cfg(debug_assertions)]
        print(&grid);
    }
    grid.into_iter().filter(|&c| c).count()
}

pub fn part_2(input: &Array2<bool>) -> usize {
    let mut grid = input.clone();
    for _ in 0..100 {
        adjust(&mut grid);
        step(&mut grid);
        #[cfg(debug_assertions)]
        print(&grid);
    }
    adjust(&mut grid);
    grid.into_iter().filter(|&c| c).count()
}
