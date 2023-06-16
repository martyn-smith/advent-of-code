///
/// Advent of Code day 2: 1202 Error
///
/*
 * The first Intcode-based problem
 */
use super::intcode::{From, Intcode};
use itertools::Itertools;

pub fn get_input() -> Vec<isize> {
    include_str!("../../data/2.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect()
}

pub fn part_1(program: &[isize]) -> usize {
    let mut program = program.to_owned();
    program[1] = 12;
    program[2] = 2;
    let mut computer = Intcode::from(&program[..]);
    computer.run(vec![]).unwrap();
    computer.intcodes[0] as usize
}

pub fn part_2(program: &[isize]) -> usize {
    let target = 19690720;
    let range = (0..100).permutations(2);
    for r in range {
        let (noun, verb) = (r[0], r[1]);
        let mut program = program.to_owned();
        program[1] = noun;
        program[2] = verb;
        let mut candidate = Intcode::from(&program[..]);
        candidate.run(vec![]).unwrap();
        if candidate.intcodes[0] as usize == target {
            return (noun * 100 + verb) as usize;
        }
    }
    panic!("no solution found!");
}
