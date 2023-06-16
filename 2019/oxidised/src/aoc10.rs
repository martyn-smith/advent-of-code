use ndarray::{Array, Array2};
//use std::fs;
use itertools::iproduct;
use num::integer::gcd;
use std::collections::HashSet;

fn count_asteroids(point: (usize, usize), asteroid_map: &Array2<bool>) -> usize {
    let p = (point.0 as i32, point.1 as i32);
    let mut sight_line = HashSet::new();
    for c in iproduct!(0..asteroid_map.nrows(), 0..asteroid_map.ncols()) {
        if !(p.0 == c.0 as i32 && p.1 == c.1 as i32) && asteroid_map[[c.0, c.1]] {
            let c = (c.0 as i32, c.1 as i32);
            let mut angle = (p.0 - c.0, p.1 - c.1);
            if angle.0 == 0 {
                angle.1 = angle.1.signum();
            } else if angle.1 == 0 {
                angle.0 = angle.0.signum();
            } else {
                let divisor = gcd(angle.0, angle.1);
                angle.0 /= divisor;
                angle.1 /= divisor;
            }
            sight_line.insert(angle);
        }
    }
    sight_line.len()
}

fn rotate(
    asteroid_map: &mut Array2<bool>,
    point: (usize, usize),
) -> Option<(usize, usize)> {
    //start by pointing straight up ((0,-1))
    //handle right and left halves separately, dealing with cardinals (-1,0) and (1,0) discretely.
    let mut polars = Vec::<(f64, f64, usize, usize)>::new();
    for c in iproduct!(0..asteroid_map.nrows(), 0..asteroid_map.ncols()) {
       let angle = (point.0 as f64 - c.0 as f64).atan2(point.1 as f64 - c.1 as f64);
       let distance = ((point.0 as f64 - c.0 as f64).abs().powi(2)
                      + (point.1 as f64 - c.1 as f64).abs().powi(2)).sqrt();
       polars.push((angle, distance, c.0, c.1));
    }
    polars.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    dbg!(&polars);
    for _ in 0..200 {
        polars.pop();
    }
    let survivor = polars.pop().unwrap();
    Some((survivor.2, survivor.3))
}

pub fn get_input() -> Array2<bool> {
    let input = include_str!("../../data/10.txt");
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
    iproduct!(0..input.nrows(), 0..input.ncols())
        .filter(|(i, j)| *input.get([*i, *j]).unwrap())
        .map(|point| count_asteroids(point, input))
        .max()
        .unwrap()
}

/*
 * The Elves are placing bets on which will be the 200th asteroid to be vaporized (starting from
 * part_1, cutting in an upward direction). Win the bet by
 * determining which asteroid that will be; what do you get if you multiply its X coordinate by 100
 * and then add its Y coordinate?
 * note points are (y,x)
 */

//923 < x < 2309
pub fn part_2(input: &Array2<bool>) -> usize {
    let mut asteroid_map = input.clone();
    let max_val = part_1(input);
    let pos = iproduct!(0..input.nrows(), 0..input.ncols())
        .filter(|(i, j)| *input.get([*i, *j]).unwrap())
        .find(|point| count_asteroids(*point, input) == max_val)
        .unwrap();
    if let Some(p) = rotate(&mut asteroid_map, pos) {
        p.0 * 100 + p.1
    } else {
        0
    }
}
