use super::intcode::Intcode;
use itertools::Itertools;

pub fn prepro(computer: &mut Intcode, noun: isize, verb: isize) {
    computer.intcodes[1] = noun;
    computer.intcodes[2] = verb;
}

pub fn get_input() -> Intcode {
    Intcode::load("../data/2.txt").unwrap()
}

pub fn part_1(program: &Intcode) -> usize {
    let mut program = program.clone();
    prepro(&mut program, 12, 2);
    program.run(vec![]).unwrap();
    program.intcodes[0] as usize
}

pub fn part_2(program: &Intcode) -> usize {
    let target = 19690720;
    let range = (0..100).permutations(2);
    for it in range {
        let mut candidate = program.clone();
        prepro(&mut candidate, it[0], it[1]);
        candidate.run(vec![]).unwrap();
        if candidate.intcodes[0] as usize == target {
            return (it[0] * 100 + it[1]) as usize;
        }
    }
    panic!("no solution found!");
}
