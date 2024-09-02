use super::alu::{Op, ALU};

fn vectorise(n: &usize) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn get_input() -> Vec<Op> {
    include_str!("../../data/24.txt")
        .lines()
        .map(Op::new)
        .collect()
}

pub fn part_1(input: &[Op]) -> usize {
    /*
     * Submarine model numbers are always fourteen-digit numbers,
     * consisting only of digits 1 through 9.
     * MONAD outputs 0 in z if valid
     */
    (0..99_999_999_999_999)
        .rev()
        .find(|n| {
            let cand = vectorise(n);
            let mut alu = ALU::new(input);
            cand.iter().all(|&i| i != 0) && alu.run(cand) == 0
        })
        .unwrap()
}
