use itertools::Itertools;

fn get_min_length(input: &[usize], target: usize) -> usize {
    let l = input.len();
    (1..l)
        .find(|&i| {
            input.iter().combinations(i)
                    .any(|c| c.into_iter().sum::<usize>() == target)
        })
        .unwrap()
    
}


pub fn get_input() -> Vec<usize> {
    include_str!("../../data/24.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[usize]) -> usize {
    let tgt = input.iter().sum::<usize>() / 3;
    let i = get_min_length(input, tgt);
    input.iter().combinations(i)
        .filter_map(|c| 
            if c.iter().map(|&&v| v).sum::<usize>() == tgt {
                Some(c.into_iter().product::<usize>())
            } else {
                None
            })
        .min().unwrap()
}


pub fn part_2(input: &[usize]) -> usize {
    let tgt = input.iter().sum::<usize>() / 4;
    let i = get_min_length(input, tgt);
    input.iter().combinations(i)
        .filter_map(|c| 
            if c.iter().map(|&&v| v).sum::<usize>() == tgt {
                Some(c.into_iter().product::<usize>())
            } else {
                None
            })
        .min().unwrap()
}
