use super::intcode::Intcode;
use itertools::Itertools;
use std::fs;

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/7.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn run_amps(intcodes: &Vec<isize>, phase: Vec<usize>) -> usize {
    //each intcode run will require two inputs: phase setting (i'th element of p),
    // plus output of last intcode run
    let mut output = 0isize;

    phase.iter().fold(0isize, |output, &p| {
        let mut computer = Intcode::new(intcodes);
        let inputs = vec![output, p as isize];
        let outputs = computer.run(inputs).unwrap();
        *outputs.last().unwrap() as isize
    }) as usize

    // for p in phase {
    //     let mut computer = Intcode::new(intcodes);
    //     let inputs = vec![output, p as isize];
    //     let outputs = computer.run(inputs).unwrap();
    //     output = *outputs.last().unwrap();
    // }
    // output as usize
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|p| run_amps(intcodes, p))
        .max()
        .unwrap()
}
