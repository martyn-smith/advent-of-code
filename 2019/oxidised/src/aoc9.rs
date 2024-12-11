use crate::intcode;
use crate::intcode::{FromStr, Program};

pub fn get_input() -> Program {
    let program = include_str!("../../data/9.txt");
    Program::from_str(program).unwrap()
}

pub fn part_1(program: &Program) -> usize {
    let program = program.clone();
    let inputs = vec![1isize];
    let outputs = intcode::solve(program, Some(inputs)).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}

pub fn part_2(program: &Program) -> usize {
    let program = program.clone();
    let inputs = vec![2isize];
    let outputs = intcode::solve(program, Some(inputs)).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}
