use cached::proc_macro::cached;

pub fn get_input() -> Vec<usize> {
    include_str!("../../data/7.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

#[cached]
fn triangle(n: usize) -> usize {
    n * (n + 1) / 2
    // if n == 0 {
    //     0
    // } else {
    //     1 + triangle(n-1)
    // }
}

fn cost(n: usize, input: &Vec<usize>) -> usize {
    input.iter()
        .map(|&i| (n as i32 - i as i32).abs() as usize)
        .sum()
}

fn tri_cost(n: usize, input: &Vec<usize>) -> usize {
    input.iter()
        .map(|&i| triangle((n as i32 - i as i32).abs() as usize))
        .sum()
}

pub fn part_1(input: &Vec<usize>) -> usize {
    let mn = *input.iter().min().unwrap();
    let mx = *input.iter().max().unwrap();
    (mn..mx)
        .map(|n| cost(n, input))
        .min()
        .unwrap()
}

pub fn part_2(input: &Vec<usize>) -> usize {
    let mn = *input.iter().min().unwrap();
    let mx = *input.iter().max().unwrap();
    (mn..mx)
        .map(|n| tri_cost(n, input))
        .min()
        .unwrap()
}
