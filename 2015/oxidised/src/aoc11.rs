use std::str;

const letters: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn get_input() -> String {
    "cqjxjnds".to_string()
}

fn increasing(pw:&str) -> bool {
    letters.as_bytes()
        .windows(3)
        .any(|w| pw.contains(str::from_utf8(&w).unwrap()))
}

fn confusing(pw: &str) -> bool {
    pw.contains('i') || pw.contains('l') || pw.contains('o')
}

fn pairs(pw: &str) -> bool {
    if let Some(a) = letters
                        .find(|c| pw.contains(&format!("{}{}", c, c)))
    {
        let mut different = letters.to_string();
        different.remove(a);
        different.chars()
                  .any(|c|
                       pw.contains(&format!("{}{}", c, c)))
    } else {
        false
    }
}

fn validate(pw: &str) -> bool {
    increasing(pw) && !confusing(pw) && pairs(pw)
}

fn increment(cand: &mut String) {
    let pairs = letters.as_bytes()
        .windows(2)
        .map(|p| str::from_utf8(&p).unwrap());
    let c = cand.pop().unwrap();
    for p in pairs {
        if c == p.chars().nth(0).unwrap() {
            cand.push(p.chars().nth(1).unwrap());
            return;
        }
    }
    increment(cand);
    cand.push('a');
}

pub fn part_1(input: &String) -> String {
    let mut cand = input.to_string();
    increment(&mut cand);
    while !validate(&cand[..]) {
        increment(&mut cand);
    }
    cand
}

pub fn part_2(input: &String) -> String {
    let mut cand = part_1(input);
    increment(&mut cand);
    while !validate(&cand[..]) {
        increment(&mut cand);
    }
    cand
}
