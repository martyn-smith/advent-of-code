use cached::proc_macro::cached;
use std::collections::HashMap;

#[cached]
fn grow(time: isize) -> usize {
    if time <= 0 {
        1
    } else {
       grow(time - 7) + grow(time - 9)
    }
}

pub fn get_input() -> Vec<usize> {
//    vec![0]
    include_str!("../../data/6.txt")
            .trim()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect()
}

pub fn part_1(input: &Vec<usize>) -> usize {
    let mut growth = HashMap::new();
    for i in 0..8usize {
        growth.insert(i, grow(80 - i as isize));
    }
    input.iter()
        .map(|i| growth.get(&i).unwrap())
        .sum()
}

pub fn part_2(input: &Vec<usize>) -> usize {
    let mut growth = HashMap::new();
    for i in 0..8usize {
        growth.insert(i, grow(256 - i as isize));
    }
    input.iter()
        .map(|i| growth.get(&i).unwrap())
        .sum()
}
