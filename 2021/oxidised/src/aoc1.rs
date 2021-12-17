///
/// Advent of Code day 1: scanning the sea floor
///

pub fn get_input() -> Vec<usize> {
    include_str!("../../data/1.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

pub fn part_1(input: &[usize]) -> usize {
    input.windows(2)
        .filter(|&p| p[0] < p[1])
        .count()
}

pub fn part_2(input: &[usize]) -> usize {
    let sums = input.windows(3)
                 .map(|t| t.iter().sum())
                 .collect::<Vec<usize>>();

    part_1(&sums)
}
