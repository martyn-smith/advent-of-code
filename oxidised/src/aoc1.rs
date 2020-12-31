use std::fs;
use std::collections::HashMap;

pub fn find_pair(target: &usize, input: &Vec<usize>) -> Option<usize> {
    let mut hashes = HashMap::new();

    for i in input.iter() {
        let complement = target - i;
        if let Some(j) = hashes.get(i) {
            return Some(i * j);
        } else {
            hashes.insert(complement as usize, *i);
        }
    }
    None
}

pub fn find_triple(target: &usize, input: &Vec<usize>) -> Option<usize> {
    for i in input.iter() {
        for j in input.iter() {
            for k in input.iter() {
                if i + j + k == *target {
                    return Some(i * j * k);
                }
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("../../1.txt").unwrap();
    let target = 2020;
    let entries = input.lines()
                       .map(|l| l.parse::<usize>().unwrap())
                       .collect();
    println!("{}", find_pair(&target, &entries).unwrap());   
    println!("{}", find_triple(&target, &entries).unwrap());
}