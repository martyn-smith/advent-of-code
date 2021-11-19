use std::fs;
use std::ops;

enum Op{
    assign(a),
    AND(a,b),
    OR(a,b),
    NOT(a),
    LSHIFT(a,b),
    RSHIFT(a,b)
}

struct Gate {
    id: String,
    op: 
}

fn parse_gate(s: &str) -> Gate {
    let left, right = s.split("->");

}

pub fn get_input() -> {
    let lines = fs::read_to_string("../data/7.txt").unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
}

pub fn part_1(diagram: &Vec<String>) -> u16 {
    for 
}
