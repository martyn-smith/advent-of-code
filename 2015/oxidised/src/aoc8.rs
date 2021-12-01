use std::fs;

fn diff(s: &String) -> usize {
    let in_mem = s.len();
    let mut repr = 0;
    let mut escape = Some(0);
    let mut s = s.chars().peekable();
    while let Some(c) = s.next() {
        if c == '\\' && escape.is_none() {
            escape = match s.peek() {
                Some('\\') => Some(0),
                Some('"') => Some(0),
                Some('x') => Some(2),
                Some(g) => panic!("unexpected non-escape char {}", g),
                None => panic!("escape character at EOL")
            };
            continue;
        }
        if let Some(ctr) = escape {
            if ctr == 0 {
                escape = None;
                repr += 1;
            } else {
                escape = Some(ctr - 1);
            }
            continue;
        }
        repr += 1;
    }
    in_mem - (repr - 2)
}

fn encode(s: &String) -> String {
    let mut out = String::new();
    let mut tmp = String::new();
    let mut escape = Some(0);
    let mut s = s.chars().peekable();
    while let Some(c) = s.next() {
        if c == '"' || c == '\\' {
            out.push(c);
        }
        out.push(c);
    }
    println!("{}", out);
    out
}

pub fn get_input() -> Vec<String> {
    fs::read_to_string("../data/8.txt").unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect()
}

pub fn part_1(input: &Vec<String>) -> usize {
    input.iter()
         .map(|l| diff(l))
         .sum()
}

//1474<x<2587
pub fn part_2(input: &Vec<String>) -> usize {
    input.iter()
         .map(|l| diff(&encode(l))+diff(l))
         .sum()
}
