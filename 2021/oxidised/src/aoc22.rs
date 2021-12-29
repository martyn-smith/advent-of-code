use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

pub type Command = (bool, [isize; 6]);

fn total_volume(dimensions: &[isize; 6]) -> usize {
    (dimensions[1] - dimensions[0]) as usize
        * (dimensions[3] - dimensions[2]) as usize
        * (dimensions[5] - dimensions[4]) as usize
}

fn intersects(point: [isize; 6], limits: [isize; 6]) -> bool {
    /*
     * six combinations, not including a "full eclipse":
     * left-down-in, right-up-out, etc
     */
    (point[0] > limits[0] && point[0] < limits[1] || point[1] > limits[0] && point[0] < limits[1])
        && (point[2] > limits[2] && point[2] < limits[3]
            || point[3] > limits[2] && point[2] < limits[3])
        && (point[4] > limits[4] && point[2] < limits[5]
            || point[4] > limits[4] && point[2] < limits[5])
}

fn additive_volume(limits: [isize; 6], cmds: &[Command]) -> usize {
    let mut cuboids = HashSet::new();
    for cmd in cmds.iter() {
        let (x_min, x_max) = (max(cmd.1[0], limits[0]), min(cmd.1[1], limits[1]));
        let (y_min, y_max) = (max(cmd.1[2], limits[2]), min(cmd.1[3], limits[3]));
        let (z_min, z_max) = (max(cmd.1[4], limits[4]), min(cmd.1[5], limits[5]));
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    if cmd.0 {
                        cuboids.insert([x, y, z]);
                    } else {
                        cuboids.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
    cuboids.len()
}

fn intersecting_cuboids(limits: [isize; 6], cmds: &[Command]) -> usize {
    let mut cuboids = HashSet::new();
    cuboids.insert(limits);
    for cmd in cmds {
        for cand in cuboids.clone().into_iter() {
            /*it doesn't actually matter whether it's an on or off cube,
            since we're counting later "on" cubes separately.
                //generate (up to) three new cuboids from intersection:
                //(x_int..x),
                //((x..x_int), (y_int..y)),
                //((x..x_int), (y..y_int), (z_int..z))
            There are 26 possible combinations;
            * x right only, (1 new cuboid)
            * x left only,
            * y down only,
            * y up only,
            * z in only,
            * z out only,
            * x right y down (2 new cuboids)
            * x left y down
            * x right y up
            * x left y up
            * x right z in
            * x left z in
            * x right z out
            * x left z out
            * y down z in
            * y up z out
            * y up z in
            * then 8 vertices (3 cuboids)
            *
            * but they can be done recursively
            */
            let mut new_cuboids = HashSet::new();
            //check x coming in from right
            if cmd.1[0] > cand[0] && cmd.1[0] < cand[1] {
                new_cuboids.insert([cmd.1[0], cand[1], cand[2], cand[3], cand[4], cand[5]]);
            } else if cmd.1[1] > cand[0] && cmd.1[1] < cand[1] {
                new_cuboids.insert([cand[0], cmd.1[1], cand[2], cand[3], cand[4], cand[5]]);
            }
            if !new_cuboids.is_empty() {
                cuboids.remove(&cand);
                cuboids.insert(*new_cuboids.iter().next().unwrap());
            }
        }
    }
    cuboids.iter().map(total_volume).sum()
}

pub fn get_input() -> Vec<Command> {
    let srch =
        Regex::new(r"(\w{2,3}) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let input = include_str!("../../data/22.txt");
    input
        .lines()
        .map(|l| {
            let cap = srch.captures(l).unwrap();
            (
                cap.get(1).unwrap().as_str() == "on",
                [
                    cap.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(5).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(6).unwrap().as_str().parse::<isize>().unwrap(),
                    cap.get(7).unwrap().as_str().parse::<isize>().unwrap(),
                ],
            )
        })
        .collect()
}

pub fn part_1(input: &[Command]) -> usize {
    additive_volume([-50, 50, -50, 50, -50, 50], input)
}

pub fn part_2(input: &[Command]) -> usize {
    /*
     * possible efficient approach for part 2:
     * return volume of each 'on' cube,
     * minus volume of any intersections from any *following* cubes,
     * of either typ (O(n^2))
     * - just return 0 from off cube
     */
    input
        .iter()
        .enumerate()
        .map(|(i, cube)| {
            if cube.0 {
                intersecting_cuboids(cube.1, &input[i..])
            } else {
                0
            }
        })
        .sum()
}
