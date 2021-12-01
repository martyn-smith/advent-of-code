use std::cmp::min;

fn make_dims(l: &str) -> (usize, usize, usize) {
    let dims = l.split('x')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (dims[0], dims[1], dims[2])
}

pub fn get_input() -> Vec<(usize, usize, usize)> {
    include_str!("../../data/2.txt")
        .lines()
        .map(|l| make_dims(l))
        .collect()
}

fn paper(dims: &(usize, usize, usize)) -> usize {
    let sides = (dims.0*dims.1, dims.1*dims.2, dims.2*dims.0);
    2*sides.0 + 2*sides.1 + 2*sides.2
    + min(min(sides.0, sides.1), sides.2)
}

fn ribbon(dims: &(usize, usize, usize)) -> usize {
    let perimeters = (2*(dims.0+dims.1), 2*(dims.1+dims.2), 2*(dims.2+dims.0));
    let volume = dims.0 * dims.1 * dims.2;
    min(min(perimeters.0, perimeters.1), perimeters.2)
    + volume
}

pub fn part_1(dims: &Vec<(usize, usize, usize)>) -> usize {
    dims.iter()
            .map(|b| paper(b))
            .sum()
}

pub fn part_2(dims: &Vec<(usize, usize, usize)>) -> usize {
    dims.iter()
            .map(|b| ribbon(b))
            .sum()
}

