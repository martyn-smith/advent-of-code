use std::fs;

struct Layer {
    data: Vec<Vec<usize>>
}

struct Image {
    data: Vec<Layer>
}

pub fn get_input() -> Vec<Vec<usize>> {
    let width = 25;
    let height = 6;
    let input = fs::read_to_string("../data/8.txt").unwrap();
    //slices
    input
        .trim()
        .chars()
        .map(|l| l.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .chunks(width * height)
        .map(|i| i.to_vec())
        .collect()
}

pub fn part_1(input: &Vec<Vec<usize>>) -> usize {
    let mut input = input.clone();
    input.sort_by(|a, b| {
        a.iter()
            .filter(|&&i| i == 0)
            .count()
            .cmp(&b.iter().filter(|&&i| i == 0).count())
    });
    input[0].iter().filter(|&&i| i == 1).count() * input[0].iter().filter(|&&i| i == 2).count()
}

pub fn part_2(input: &Vec<Vec<usize>>) {

}
