use itertools::Itertools;
///
/// Advent of Code day 8: seven-segment displays make a pretty graph
///
/*
 * Starts out as a bipartite graph with 7! (5040) possibilities (?)
 * Given we always have all 10 numbers present, a deterministic solution should be possible.
 *
 * n | 0 1 2 3 4 5 6 7 8 9
 * -----------------------
 * l | 6 2 5 5 4 5 6 3 7 5
 *
 * First, identify 1, 4, 7, 8 by length;
 * 9 can be idenfitied as containing only the chars in 4 and 7
 *   (i.e. 4 + 7 == 9)
 * x->a can be determined as the only char in 7 and not 1
 * x->b can be determined as occurring in 4 and 9 and not 7
 * x->d can be determined as occurring only in 4 and not 1 or 7
 * x->e cen be determined as occurring in 8 and not 9
 * 6 can be identified as containing a and d and being of length 6
 * 0 can be identified as not containing d and being of length 6
 * x->c can be determined as being in 1 and not 6.
 * x->f can be determined as being in 1 and not being c.
 * x->g is the remainder
 */
use std::collections::HashMap;

const NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

pub type Display = (Vec<String>, Vec<String>);

fn count_1478(line: &[String]) -> usize {
    line.iter().filter(|&l| by_len(l).is_some()).count()
}

fn by_len(s: &str) -> Option<usize> {
    match s.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn intersect(x: &str, y: &str) -> usize {
    x.chars().filter(|&c| y.contains(c)).count()
}

fn by_intersection(x: &str, nums: &[Option<&str>; 10]) -> usize {
    match (
        intersect(x, nums[1].unwrap()),
        intersect(x, nums[4].unwrap()),
        intersect(x, nums[7].unwrap()),
        x.len(),
    ) {
        (2, 3, 3, 6) => 0,
        (_, _, _, 2) => 1,
        (1, 2, 2, 5) => 2,
        (2, 3, 3, 5) => 3,
        (_, _, _, 4) => 4,
        (1, 3, 2, 5) => 5,
        (1, 3, 2, 6) => 6,
        (_, _, _, 3) => 7,
        (_, _, _, 7) => 8,
        (2, 4, 3, 6) => 9,
        _ => unreachable!(),
    }
}

fn solve(line: &[String]) -> HashMap<&str, usize> {
    let mut nums: [Option<&str>; 10] = [None; 10];
    let mut seg = HashMap::new();
    for l in line.iter() {
        if let Some(i) = by_len(&l[..]) {
            nums[i] = Some(&l[..]);
        }
    }

    for i in [0, 2, 3, 5, 6, 9].iter() {
        for l in line.iter() {
            if by_intersection(l, &nums) == *i {
                nums[*i] = Some(&l[..]);
                break;
            }
        }
    }
    for (i, n) in nums.iter().enumerate() {
        seg.insert(n.unwrap(), i);
    }
    seg
}
fn get_line(l: &str) -> Display {
    let mut s = l.split('|');
    let samples = s
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.chars().sorted().collect::<String>());
    let values = s
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.chars().sorted().collect::<String>());
    (samples.collect(), values.collect())
}

pub fn get_input() -> Vec<(Vec<String>, Vec<String>)> {
    include_str!("../../data/8.txt")
        .lines()
        .map(get_line)
        .collect()
}

pub fn part_1(input: &[Display]) -> usize {
    input.iter().map(|l| count_1478(&l.1)).sum()
}

pub fn part_2(input: &[Display]) -> usize {
    input
        .iter()
        .map(|l| {
            let m = solve(&l.0);
            let mut s = String::new();
            for k in l.1.iter() {
                let v = m.get(&k[..]).unwrap();
                s.push_str(&v.to_string()[..]);
            }
            s.parse::<usize>().unwrap()
        })
        .sum()
}
