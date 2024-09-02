///
/// Advent of Code day 10: bracket matching (Ye Olde Stacke)
///

/*
Rust is useless at static arrays, so these are useless too. Kept as documentation.
const OPENING: [char; 4] = ['(','[','{','<'];
const CLOSING: [char; 4] = [')',']','}','>'];
*/

fn illegal_char(line: &str) -> Option<char> {
    let mut close = vec![];
    for c in line.chars() {
        match c {
            '(' => {
                close.push(')');
            }
            '[' => {
                close.push(']');
            }
            '{' => {
                close.push('}');
            }
            '<' => {
                close.push('>');
            }
            //HAZMAT: this is not robust against chars other than  [')',']','}','>'];
            _ => {
                if close.pop() == Some(c) {
                } else {
                    return Some(c);
                }
            }
        }
    }
    None
}

fn completion_chars(line: &str) -> Option<String> {
    let mut close = vec![];
    for c in line.chars() {
        match c {
            '(' => {
                close.push(')');
            }
            '[' => {
                close.push(']');
            }
            '{' => {
                close.push('}');
            }
            '<' => {
                close.push('>');
            }
            //HAZMAT: this is not robust against chars other than  [')',']','}','>'];
            _ => {
                if close.pop() == Some(c) {
                } else {
                    return None;
                }
            }
        }
    }
    Some(close.into_iter().rev().collect())
}

fn score_corrupted(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_completion(s: String) -> usize {
    s.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    })
}

pub fn get_input() -> Vec<String> {
    include_str!("../../data/10.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

pub fn part_1(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|l| illegal_char(l))
        .map(score_corrupted)
        .sum()
}

pub fn part_2(input: &[String]) -> usize {
    let mut scores = input
        .iter()
        .filter_map(|l| completion_chars(l))
        .map(score_completion)
        .collect::<Vec<usize>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
