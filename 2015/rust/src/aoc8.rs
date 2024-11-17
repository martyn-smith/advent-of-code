//use std::fs;

fn decode(s: &str) -> String {
    let s = &s[1..s.len() - 1];
    let mut out = String::new();
    let mut p = s.chars();
    while let Some(c) = p.next() {
        if c == '\\' {
            match p.next().expect("illegal escape character position") {
                '\\' => {
                    out.push('\\');
                }
                '"' => {
                    out.push('"');
                }
                'x' => {
                    let n = u8::from_str_radix(
                        &format!(
                            "{}{}",
                            p.next().expect("illegal hex character position"),
                            p.next().expect("illegal hex character position")
                        )[..],
                        16,
                    )
                    .expect("couldn't parse as byte");
                    if n.is_ascii_digit() {
                        out.push(char::from(n));
                    } else {
                        out.push('X');
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn encode(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => {
                out.push_str("\\\"");
            }
            '\\' => {
                out.push_str("\\\\");
            }
            _ => {
                out.push(c);
            }
        }
    }
    format!("\"{}\"", out)
}

pub fn get_input() -> Vec<String> {
    include_str!("../../data/8.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

pub fn part_1(input: &[String]) -> usize {
    input.iter().map(|l| l.len() - decode(l).len()).sum()
}

pub fn part_2(input: &[String]) -> usize {
    input.iter().map(|l| encode(l).len() - l.len()).sum()
}
