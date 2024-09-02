///
/// Advent of Code day 7: Crab Submarines
///

fn triangle(n: usize) -> usize {
    n * (n + 1) / 2
}

fn cost(n: usize, input: &[usize]) -> usize {
    input
        .iter()
        .map(|&i| (n as isize - i as isize).unsigned_abs())
        .sum()
}

fn tri_cost(n: usize, input: &[usize]) -> usize {
    input
        .iter()
        .map(|&i| triangle((n as isize - i as isize).unsigned_abs()))
        .sum()
}

pub fn get_input() -> Vec<usize> {
    include_str!("../../data/7.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

pub fn part_1(input: &[usize]) -> usize {
    let mut input = input.to_owned();
    input.sort_unstable();
    let median = input[input.len() / 2];
    (median - 1..=median + 1)
        .map(|n| cost(n, &input))
        .min()
        .unwrap()
}

pub fn part_2(input: &[usize]) -> usize {
    let mean = input.iter().sum::<usize>() / input.len();
    (mean - 1..=mean + 1)
        .map(|n| tri_cost(n, input))
        .min()
        .unwrap()
}
