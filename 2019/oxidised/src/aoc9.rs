use super::intcode::Intcode;
use std::fs;

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/9.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let mut computer = Intcode::new(intcodes);
    let system_id = 1isize;
    let inputs = vec![system_id];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}

pub fn part_2(intcodes: &Vec<isize>) -> usize {
    let mut computer = Intcode::new(intcodes);
    let system_id = 2isize;
    let inputs = vec![system_id];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}
