///
/// Advent of Code day 6: All the lanternfish
///
use cached::proc_macro::cached;
use std::collections::HashMap;

/*
 * Since a lanternfish cannot be more than 8 days old,
 * there are clearly only 8 solutions at time t.
 *
 * Starting at zero, population cycles:
 *
 *   day   |    pop
 *   --------------
 *   0        1
 *   1        6 8
 *   ...      ...
 *   7        0 2
 *
 *   leading to a functional relation:
 *
 *   g(0,0)     => 1
 *   g(0,1..=7) => 2
 *   g(0,t)     => g(0,t-7) + g(2,t-7)
 *   g(n,t)     => g(0,t-n) //<-check this sub for underflow
 */
#[cached]
fn g(i: usize, t: usize) -> usize {
    if i > 0 {
        g(0, t.saturating_sub(i))
    } else if t == 0 {
        1
    } else if t < 7 {
        2
    } else {
        g(0, t - 7) + g(2, t - 7)
    }
}

pub fn get_input() -> Vec<usize> {
    include_str!("../../data/6.txt")
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

pub fn part_1(input: &[usize]) -> usize {
    let mut growth = HashMap::new();
    for i in 0..8 {
        growth.insert(i, g(i, 80));
    }
    input.iter().map(|i| growth.get(i).unwrap()).sum()
}

pub fn part_2(input: &[usize]) -> usize {
    let mut growth = HashMap::new();
    for i in 0..8 {
        growth.insert(i, g(i, 256));
    }
    input.iter().map(|i| growth.get(i).unwrap()).sum()
}
