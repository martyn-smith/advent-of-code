///
/// Advent of Code day 2: 1202 Error
///
/*
 * The first Intcode-based problem
 */
use super::intcode::Intcode;
use itertools::Itertools;

pub fn get_input() -> Vec<isize> {
    include_str!("../../data/2.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect()
}

pub fn part_1(program: &Vec<isize>) -> usize {
    let mut program = program.clone();
    program[1] = 12;
    program[2] = 2;
    let mut computer = Intcode::new(&mut program);
    computer.run(vec![]);
    program[0] as usize
}

pub fn part_2(program: &Vec<isize>) -> usize {
    let target = 19690720;
    let range = (0..100).permutations(2);
    for r in range {
        let (noun, verb) = (r[0], r[1]);
        let mut candidate = program.clone();
        candidate[1] = noun;
        candidate[2] = verb;
        let mut computer = Intcode::new(&mut candidate);
        computer.run(vec![]);
        if candidate[0] as usize == target {
            return (noun * 100 + verb) as usize;
        }
    }
    panic!("no solution found!");
}
