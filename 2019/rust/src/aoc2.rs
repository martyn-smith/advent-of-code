///
/// Advent of Code day 2: 1202 Error
///
/*
 * The first Intcode-based problem.
 * This appears to be the only one where the program is relevant,
 * so the solution must own the program as a primitive throughout.
 */
use crate::intcode::{Computer, From, Program};
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
    let mut program = Program::from(&program[..]);
    let mut computer = Computer::new();
    computer.run(&mut program, None).unwrap();
    program.intcodes[0] as usize
}

pub fn part_2(program: &[isize]) -> usize {
    let target = 19690720;
    let range = (0..100).permutations(2);
    for r in range {
        let (noun, verb) = (r[0], r[1]);
        let mut program = program.to_owned();
        program[1] = noun;
        program[2] = verb;
        let mut candidate = Program::from(&program[..]);
        let mut computer = Computer::new();
        computer.run(&mut candidate, None).unwrap();
        if candidate.intcodes[0] as usize == target {
            return (noun * 100 + verb) as usize;
        }
    }
    panic!("no solution found!");
}
