use std::collections::HashMap;

pub fn get_input() -> (Vec<usize>, Vec<usize>) {
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();
    for l in include_str!("../../data/1.txt").lines() {
        let mut pair = l.split_whitespace();
        a.push(pair.next().unwrap().parse::<usize>().unwrap());
        b.push(pair.next().unwrap().parse::<usize>().unwrap());
    }
    (a, b)
}

pub fn part_1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left = input.0.clone();
    let mut right = input.1.clone();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l as i32 - r as i32).abs())
        .sum::<i32>() as usize
}

pub fn part_2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let left = input.0.clone();
    let mut right = HashMap::<usize, usize>::new();
    for &n in input.1.iter() {
        right
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    left.into_iter()
        .map(|l| l * right.get(&l).unwrap_or(&0))
        .sum::<usize>()
}
