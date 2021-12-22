use std::collections::HashSet;
use std::cmp::{max, min};
use regex::Regex;

pub type Command = (bool, [isize; 6]);

pub fn get_input() -> Vec<Command> {
    let srch = Regex::new(r"(\w{2,3}) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let input = include_str!("../../data/22.txt");
    input.lines()
        .map(|l| {
            let cap = srch.captures(l).unwrap();
            (cap.get(1).unwrap().as_str() == "on",
             [cap.get(2).unwrap().as_str().parse::<isize>().unwrap(),
              cap.get(3).unwrap().as_str().parse::<isize>().unwrap(),
              cap.get(4).unwrap().as_str().parse::<isize>().unwrap(),
              cap.get(5).unwrap().as_str().parse::<isize>().unwrap(),
              cap.get(6).unwrap().as_str().parse::<isize>().unwrap(),
              cap.get(7).unwrap().as_str().parse::<isize>().unwrap()])
        })
        .collect()
}

pub fn part_1(input: &[Command]) -> usize {
    let mut cubes = HashSet::new();
    for i in input.iter() {
        let (x_min, x_max) = (max(i.1[0], -50), min(i.1[1], 50));
        let (y_min, y_max) = (max(i.1[2], -50), min(i.1[3], 50));
        let (z_min, z_max) = (max(i.1[4], -50), min(i.1[5], 50));
        for x in (x_min..=x_max) {
            for y in (y_min..=y_max) {
                for z in (z_min..=z_max) {
                    if i.0 {
                        cubes.insert([x, y, z]);
                    } else {
                        cubes.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
    cubes.len()
}

pub fn part_2(input: &[Command]) -> usize {
    let mut cubes = HashSet::new();
    for i in input.iter() {
        let (x_min, x_max) = (i.1[0], i.1[1]);
        let (y_min, y_max) = (i.1[2], i.1[3]);
        let (z_min, z_max) = (i.1[4], i.1[5]);
        for x in (x_min..=x_max) {
            for y in (y_min..=y_max) {
                for z in (z_min..=z_max) {
                    if i.0 {
                        cubes.insert([x, y, z]);
                    } else {
                        cubes.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
    cubes.len()
}
