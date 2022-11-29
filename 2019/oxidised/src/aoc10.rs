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
    ctr: &mut usize,
    start: &(usize, usize),
) -> Option<(usize, usize)> {
    //start by pointing straight up ((0,-1))
    //handle right and left halves separately, dealing with cardinals (-1,0) and (1,0) discretely.

    //handle N cardinal
    for y in (0..start.1).rev() {
        if asteroid_map[[start.0, y]] {
            *asteroid_map.get_mut([start.0, y]).unwrap() = false;
            *ctr += 1;
            if *ctr == 200 {
                return Some((0, 0));
            } else {
                break;
            }
        }
    }
    //NE sector
    for _a in iproduct!(
        start.0 + 1..asteroid_map.ncols(),
        0..start.1
        )
        .filter(|c| asteroid_map[[c.0, c.1]])
        .collect::<Vec<(usize, usize)>>()
    {
        //collect asteroids and sort by angle


    }
    //handle E cardinal
    for _x in start.0 + 1..asteroid_map.ncols() {}
    //SE sector
    for _c in iproduct!(
        start.0 + 1..asteroid_map.ncols(),
        start.1 + 1..asteroid_map.nrows()
    ) {}
    //handle S cardinal
    for y in start.1 + 1..asteroid_map.nrows() {
        if asteroid_map[[y, start.1]] {
            *asteroid_map.get_mut([y, start.1]).unwrap() = false;
            *ctr += 1;
            break;
        }
    }
    if *ctr == 200 {
        return Some((0, 0));
    }
    //SW sector
    for _c in iproduct!(
        (0..start.0).rev(),
        (start.1 + 1..asteroid_map.nrows()).rev()
    ) {}
    //handle W cardinal
    for _x in (0..start.0).rev() {}
    //NW sector
    for _c in iproduct!(0..start.0, (0..start.1).rev()) {}
    None
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

pub fn part_2(input: &Array2<bool>) -> usize {
    let mut asteroid_map = input.clone();
    let mut destroyed = 0;
    let max_val = part_1(input);
    let pos = iproduct!(0..input.nrows(), 0..input.ncols())
        .filter(|(i, j)| *input.get([*i, *j]).unwrap())
        .find(|point| count_asteroids(*point, input) == max_val)
        .unwrap();
    if let Some(p) = rotate(&mut asteroid_map, &mut destroyed, &pos) {
        p.1 * 100 + p.0
    } else {
        0
    }
}
