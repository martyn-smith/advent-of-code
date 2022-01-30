use super::intcode::{Intcode, Program, from_str};

pub fn get_input() -> Program {
    from_str(include_str!("../../data/5.txt"))
}

pub fn part_1(p: &Program) -> usize {
    let mut p = p.clone();
    let mut computer = Intcode::new(&mut p);
    let system_id = 1isize;
    let inputs = vec![system_id];
    computer.inputs = inputs;
    computer.last().unwrap() as usize
}

pub fn part_2(p: &Program) -> usize {
    let mut p = p.clone();
    let mut computer = Intcode::new(&mut p);
    let system_id = 5isize;
    let inputs = vec![system_id];
    computer.inputs = inputs;
    computer.last().unwrap() as usize
}
