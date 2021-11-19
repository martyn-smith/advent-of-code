use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("../data/1.txt").unwrap()
}

pub fn part_1(instructions: &String) -> usize {
    instructions.chars()
                .filter(|&c| c == '(')
                .count()
    - instructions.chars()
                .filter(|&c| c == ')')
                .count()
}

pub fn part_2(instructions: &String) -> usize {
    let mut floor: i32 = 0;
    for (i, ins) in instructions.chars().enumerate() {
        floor += match ins {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        };
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("no solution found");
}