use super::intcode::Intcode;
use itertools::Itertools;
use std::fs;

fn prepro(intcodes: &mut Vec<isize>, noun: isize, verb: isize) {
    intcodes[1] = noun;
    intcodes[2] = verb;
}

fn run_intcode(mut intcodes: Vec<isize>) -> Result<usize, usize> {
    let mut computer = Intcode::new(&intcodes);
    let inputs: Vec<isize> = vec![];
    let _ = computer.run(inputs).unwrap();
    Ok(computer.intcodes[0] as usize)
}

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/2.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(program: &Vec<isize>) -> Result<usize, usize> {
    let mut program = program.clone();
    prepro(&mut program, 12, 2);
    run_intcode(program)
}

pub fn part_2(program: &Vec<isize>) -> Option<usize> {
    let target = 19690720;
    let range = (79..80).cartesian_product(12..13);
    for it in range {
        let mut candidate = program.clone();
        prepro(&mut candidate, 79, 12);
        if let Ok(r) = run_intcode(candidate) {
            if r == target {
                return Some(it.0 * 100 + it.1);
            }
        }
    }
    None
}
