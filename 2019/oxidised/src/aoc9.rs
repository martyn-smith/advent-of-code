use super::intcode::Intcode;

pub fn get_input() -> Intcode {
    Intcode::load("../data/9.txt").unwrap()
}

pub fn part_1(computer: &Intcode) -> usize {
    let mut computer = computer.clone();
    let inputs = vec![1isize];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}

pub fn part_2(computer: &Intcode) -> usize {
    let mut computer = computer.clone();
    let inputs = vec![2isize];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}
