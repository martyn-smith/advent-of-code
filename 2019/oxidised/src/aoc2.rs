use super::intcode::Intcode;
use itertools::Itertools;
use std::fs;

fn prepro(intcodes: &mut Vec<isize>, noun: isize, verb: isize) {
    intcodes[1] = noun;
    intcodes[2] = verb;
}

fn run_intcode(intcodes: Vec<isize>) -> usize {
    let mut computer = Intcode::new(&intcodes);
    let inputs: Vec<isize> = vec![];
    let _ = computer.run(inputs).unwrap();
    computer.intcodes[0] as usize
}

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/2.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(program: &Vec<isize>) -> usize {
    let mut program = program.clone();
    prepro(&mut program, 12, 2);
    run_intcode(program)
}

pub fn part_2(program: &Vec<isize>) -> usize {
    let target = 19690720;
    let range = (0..100).permutations(2);
    for it in range {
        let mut candidate = program.clone();
        prepro(&mut candidate, it[0], it[1]);
        if run_intcode(candidate) == target {
            return (it[0] * 100 + it[1]) as usize;
        }
    }
    panic!("no solution found!");
}
