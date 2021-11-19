use std::fs;
use regex::Regex;

pub fn get_input() -> Vec<String> {
    fs::read_to_string("../data/5.txt").unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn is_nice(s: &String) -> bool {
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let repeat = Regex::new(r"[(a-z)]\1").unwrap();
    let naughty = Regex::new(r"(ab)|(cd)|(pq)|(xy)").unwrap();

    vowels.find_iter(s).count() >= 3
    && repeat.is_match(s)
    && !naughty.is_match(s)
}

// < 575
pub fn part_1(strings: &Vec<String>) -> usize {
    strings.iter()
        .filter(|s| is_nice(s))
        .count()
}
