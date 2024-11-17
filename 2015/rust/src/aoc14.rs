///
/// Advent of Code day 14: Modular racing with Reindeer
///
use anyhow::Result;
//use std::fs;
use regex::{Captures, Regex};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Reindeer {
    name: String,
    speed: usize,
    duration: usize,
    cooldown: usize,
    x: usize,
    points: usize,
}

impl Reindeer {
    fn new(captures: Captures) -> Result<Self> {
        Ok(Reindeer {
            name: captures[1].to_string(),
            speed: captures[2].parse::<usize>()?,
            duration: captures[3].parse::<usize>()?,
            cooldown: captures[4].parse::<usize>()?,
            x: 0,
            points: 0,
        })
    }

    fn travel(&self, seconds: usize) -> usize {
        let cycles = seconds / (self.duration + self.cooldown);
        (self.speed * self.duration * cycles) + (self.speed * min(seconds % cycles, self.duration))
    }

    fn step(&mut self, seconds: usize) {
        if seconds % (self.duration + self.cooldown) < self.duration {
            self.x += self.speed;
        }
    }
}

pub fn get_input() -> Vec<Reindeer> {
    let input = include_str!("../../data/14.txt");
    let re = Regex::new(r"^(\w.*) .* fly ([0-9]+) .* for ([0-9]+) .* for ([0-9]+) .*$").unwrap();
    input
        .lines()
        .map(|l| Reindeer::new(re.captures(l).unwrap()).unwrap())
        .collect()
}

pub fn part_1(input: &[Reindeer]) -> usize {
    input.iter().map(|r| r.travel(2503)).max().unwrap()
}

pub fn part_2(input: &[Reindeer]) -> usize {
    let mut input = input.to_owned();
    for t in 0..2503 {
        for r in input.iter_mut() {
            r.step(t);
        }

        let lead = input.iter().map(|r| r.x).max().unwrap();

        for r in input.iter_mut().filter(|r| r.x == lead) {
            r.points += 1;
        }
    }

    input.iter().map(|r| r.points).max().unwrap()
}
