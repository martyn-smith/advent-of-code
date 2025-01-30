use std::cmp::Ordering;

#[derive(Debug)]
enum ReportState {
    First(usize),
    Increasing(usize),
    Decreasing(usize),
}

fn walk(levels: &[usize]) -> bool {
    let mut state = None;
    for &b in levels.iter() {
        state = match state {
            None => Some(ReportState::First(b)),
            Some(ReportState::First(a)) => match b.cmp(&a) {
                Ordering::Greater => {
                    if b > a + 3 {
                        return false;
                    } else {
                        Some(ReportState::Increasing(b))
                    }
                }
                Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    if b < a - 3 {
                        return false;
                    } else {
                        Some(ReportState::Decreasing(b))
                    }
                }
            },
            Some(ReportState::Decreasing(a)) => {
                if b >= a || b < a - 3 {
                    return false;
                } else {
                    Some(ReportState::Decreasing(b))
                }
            }
            Some(ReportState::Increasing(a)) => {
                if b <= a || b > a + 3 {
                    return false;
                } else {
                    Some(ReportState::Increasing(b))
                }
            }
        }
    }
    true
}

fn dampened_walk(levels: &Vec<usize>) -> bool {
    let mut dampened = (0..levels.len())
        .map(|i| {
            let mut l = Vec::<usize>::new();
            l.extend_from_slice(&levels[0..i]);
            l.extend_from_slice(&levels[i + 1..]);
            l
        })
        .collect::<Vec<_>>();
    dampened.push(levels.to_owned());
    dampened.iter().any(|r| walk(r))
}

pub fn get_input() -> Vec<Vec<usize>> {
    include_str!("../../data/2.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|r| walk(r)).count()
}

pub fn part_2(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|r| dampened_walk(r)).count()
}
