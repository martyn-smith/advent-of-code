use crate::intcode;
use crate::intcode::{FromStr, Program};

pub fn get_input() -> Program {
    Program::from_str(include_str!("../../data/5.txt")).unwrap()
}

pub fn part_1(program: &Program) -> usize {
    let program = program.clone();
    let system_id = 1isize;
    let inputs = vec![system_id];
    let outputs = intcode::solve(program, Some(inputs)).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}

pub fn part_2(program: &Program) -> usize {
    let program = program.clone();
    let system_id = 5isize;
    let inputs = vec![system_id];
    let outputs = intcode::solve(program, Some(inputs)).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}
