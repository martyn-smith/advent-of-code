use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
pub enum Op{
    AND(String, String),
    OR(String, String),
    NOT(String),
    LSHIFT(String, String),
    RSHIFT(String, String),
    NULL(String),
    //technically not necessary, but saves time
    VAL(u16)
}

fn is_solved(x: String, gates: &HashMap<String, Op>) -> Option<u16> {
    if let Ok(x) = u16::from_str_radix(a, 10) {
        Some(x)
    } else {
        if let Op::Val(x) = gates.get(x).expect("not a number or key!") {
            Some(x)
        } else {
            None
        }
    }
}

fn solve(gates: &mut HashMap<String, Op>, id: &str) -> Result<()> {
    //try to parse the operand as a value. If it succeeds, do no more;
    //otherwise, solve the subtree and retry
    //println!("solving {}", id);
    let to_solve = match gates.get(id).ok_or(anyhow!("no key"))? {
        Op::VAL(_) => {
            vec![]
        },
        Op::NULL(s) | Op::NOT(s) => {
            if let None = is_solved(s, gates) {
                vec![s.to_string()]
            } else {
                vec![]
            }
        },
        Op::AND(a, b) | Op::OR(a, b) | Op::LSHIFT(a, b) | Op::RSHIFT(a, b) => {
            match (is_solved(a, gates), is_solved(b, gates)) {
                (None, None) => {
                    vec![a.to_string(), b.to_string()]
                },
                (None, Some(_)) => {
                    vec![b.to_string()]
                },
                (Some(_), None) => {
                    vec![a.to_string()]
                },
                (Some(_), Some(_)) => {
                    vec![]
                },
            }
        }
    };
    for s in to_solve.iter() {
        solve(gates, &s[..]);
    }
    let g = gates.get_mut(id).ok_or(anyhow!("no key"))?;
    match g {
        Op::VAL(_) => {
            Ok(())
        },
        Op::NULL(s) => {
            let s = gates.get(id);
            *g = Op::VAL(s);
            Ok(())
        },
        Op::RSHIFT(a, b) => {
            let a = u16::from_str_radix(a, 10)?;
            let b = u16::from_str_radix(b, 10)?;
            *g = Op::VAL(a >> b);
            Ok(())
        }
        Op::LSHIFT(a, b) => {
            let a = u16::from_str_radix(a, 10)?;
            let b = u16::from_str_radix(b, 10)?;
            *g = Op::VAL(a << b);
            Ok(())
        }
        Op::AND(a, b) => {
            let a = u16::from_str_radix(a, 10)?;
            let b = u16::from_str_radix(b, 10)?;
            *g = Op::VAL(a & b);
            Ok(())
        }
        Op::OR(a, b) => {
            let a = u16::from_str_radix(a, 10)?;
            let b = u16::from_str_radix(b, 10)?;
            *g = Op::VAL(a | b);
            Ok(())
        }
        Op::NOT(s) => {
            let s = u16::from_str_radix(s, 10)?;
            *g = Op::VAL(!s);
            Ok(())
        }
    }
}

fn get_operation(left: &str) -> Op {
    if left.contains("NOT") {
        let operands = left.split("NOT ").collect::<Vec<&str>>();
        Op::NOT(operands[1].to_string())
    }
    else if left.contains("AND") {
        let operands = left.split(" AND ").collect::<Vec<&str>>();
        Op::AND(operands[0].to_string(), operands[1].to_string())
    }
    else if left.contains("OR") {
        let operands = left.split(" OR ").collect::<Vec<&str>>();
        Op::OR(operands[0].to_string(), operands[1].to_string())
    }
    else if left.contains("LSHIFT") {
        let operands = left.split(" LSHIFT ").collect::<Vec<&str>>();
        Op::LSHIFT(operands[0].to_string(), operands[1].to_string())
    }
    else if left.contains("RSHIFT") {
        let operands = left.split(" RSHIFT ").collect::<Vec<&str>>();
        Op::RSHIFT(operands[0].to_string(), operands[1].to_string())
    }
    else if let Ok(signal) = u16::from_str_radix(left, 10) {
        Op::VAL(signal)
    } else {
        Op::NULL(left.to_string())

    }
}

pub fn get_input() -> HashMap<String, Op> {
    let lines = fs::read_to_string("../data/7.txt").unwrap();
    let mut gates = HashMap::new();
    for l in lines.lines() {
        let cmd = l.split(" -> ").collect::<Vec<&str>>();
        let (left, id) = (cmd[0], cmd[1]);
        let inputs = get_operation(left);
        gates.insert(id.to_string(), inputs);
    }
    gates
}

pub fn part_1(gates: &HashMap<String, Op>) -> u16 {
    let mut g = gates.clone();
    solve(&mut g, "a").unwrap();
    match gates.get("a").unwrap() {
        Op::VAL(x) => *x,
        _ => panic!("unable to solve")
    }
}
