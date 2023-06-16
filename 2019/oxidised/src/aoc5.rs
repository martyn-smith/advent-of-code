use super::intcode::{FromStr, Intcode};

pub fn get_input() -> Intcode {
    Intcode::from_str(include_str!("../../data/5.txt")).unwrap()
}

pub fn part_1(computer: &Intcode) -> usize {
    let mut computer = computer.clone();
    let system_id = 1isize;
    let inputs = vec![system_id];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}

pub fn part_2(computer: &Intcode) -> usize {
    let mut computer = computer.clone();
    let system_id = 5isize;
    let inputs = vec![system_id];
    let outputs = computer.run(inputs).unwrap();
    assert!(
        outputs.iter().filter(|&&i| i != 0).count() <= 1,
        "ERROR: intcode returned too many non-zero status codes"
    );
    *outputs.last().unwrap() as usize
}
