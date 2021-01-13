use itertools::Itertools;
use std::fs;
use super::intcode::Intcode;

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/7.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn run_amps(intcodes: &Vec<isize>, phase: Vec<usize>) -> usize {
    //each intcode run will require two inputs: phase setting (i'th element of p), and input 9
    //(output of last intcode run, plus 
    let mut output = 0isize;
    //println!("testing {:?}", phase);
    for p in phase {
        let mut computer = Intcode::new(intcodes);
        let inputs = vec![output, p as isize];
        //println!("inputs: {:?}", inputs);
        let outputs = computer.run(inputs).unwrap();
        output = *outputs.last().unwrap();
    }
    output as usize
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|p| run_amps(intcodes, p))
        .max()
        .unwrap()
}