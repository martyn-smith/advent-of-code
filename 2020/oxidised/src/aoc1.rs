use std::collections::HashSet;
use std::fs;

pub fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("../data/1.txt").unwrap();
    input.lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn find_pair(target:usize, nums: &Vec<usize>) -> Option<(usize, usize)> {
    let mut prev = HashSet::new();
    nums.iter()
        .map(|&num| {
            let comp = target - num;
            if prev.contains(&comp) {
                Some((num, comp))
            } else {
                prev.insert(num);
                None
            }
        })
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .next()  
}

pub fn part_1(target: usize, nums: &Vec<usize>) -> usize {
      let pair = find_pair(target, nums).unwrap();
      pair.0 * pair.1
}

pub fn part_2(target: usize, nums: &Vec<usize>) -> usize {
    for n in nums.iter() {
        let target = target - n;
        if let Some(p) = find_pair(target, nums) {
            return n * p.0 * p.1;
        }
    }
    unreachable!();
}
