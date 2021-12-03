//used as a bit of a hack, Rust only has an integer log() as nightly.
const MAX_BASE: u16 = 12;

pub fn get_input() -> Vec<u16> {
    include_str!("../../data/3.txt")
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

fn count_ones(pos:u16, input: &Vec<u16>) -> usize {
    input.iter()
         .filter(|&i| (i >> pos) & 1 == 1)
         .count()
}

fn count_zeros(pos:u16, input: &Vec<u16>) -> usize {
    input.iter()
          .filter(|&i| (i >> pos) & 1 == 0)
          .count()
}

fn most_common(pos: u16, input: &Vec<u16>) -> u16 {
    if count_ones(pos, input) >= count_zeros(pos, input) {1} else {0}
}

fn least_common(pos: u16, input: &Vec<u16>) -> u16 {
    if count_ones(pos, input) < count_zeros(pos, input) {1} else {0}
}

fn epsilon(input: &Vec<u16>) -> usize {
    (0..MAX_BASE)
        .fold(0usize, |acc, pos| {
            let r = least_common(pos, input);
            acc + (r << pos) as usize
        })
}

fn gamma(input: &Vec<u16>) -> usize {
    (0..MAX_BASE)
        .fold(0usize, |acc, pos| {
            let r = most_common(pos, input);
            acc + (r << pos) as usize
        })
}

fn oxygen(input: &Vec<u16>) -> usize {
    let mut pos = MAX_BASE;
    let mut cand = input.clone();
    while cand.len() > 1 {
        pos -= 1;
        let r = most_common(pos, &cand);
        cand.retain(|&i| (i >> pos) & 1 == r);
    }
    cand[0] as usize
}

fn co2(input: &Vec<u16>) -> usize {
    let mut pos = MAX_BASE;
    let mut cand = input.clone();
    while cand.len() > 1 {
        pos -= 1;
        let r = least_common(pos, &cand);
        cand.retain(|&i| (i >> pos) & 1 == r);
    }
    cand[0] as usize
}

pub fn part_1(input: &Vec<u16>) -> usize {
    gamma(input) * epsilon(input)
}

pub fn part_2(input: &Vec<u16>) -> usize {
    oxygen(&input) * co2(&input)
}
