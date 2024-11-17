use anyhow::Result;
use std::collections::HashMap;
//use std::fs;

#[derive(Clone, Debug)]
pub enum MaybeInt {
    Solved(u16),
    Unsolved(String),
}

impl MaybeInt {
    fn new(left: &str) -> Self {
        if let Ok(x) = left.parse::<u16>() {
            Self::Solved(x)
        } else {
            Self::Unsolved(left.to_string())
        }
    }
}

#[derive(Debug)]
enum OpCodes {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
    Assign,
}

#[derive(Debug)]
struct Op {
    op: OpCodes,
    val: Vec<u16>,
}

fn get_u16(gates: &HashMap<String, MaybeInt>, operand: &str) -> u16 {
    if let Ok(v) = operand.parse::<u16>() {
        v
    } else if let Some(MaybeInt::Solved(v)) = gates.get(operand) {
        *v
    } else {
        panic!()
    }
}

impl Op {
    fn new(gates: &HashMap<String, MaybeInt>, left: &MaybeInt) -> Self {
        match left {
            MaybeInt::Solved(v) => {
                //I don't think we should ever be here...
                Self {
                    op: OpCodes::Assign,
                    val: vec![*v],
                }
            }
            MaybeInt::Unsolved(lhs) => {
                if lhs.contains("Not") {
                    let operands = lhs.split("Not ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::Not,
                        val: vec![get_u16(gates, operands[1])],
                    }
                } else if lhs.contains("And") {
                    let operands = lhs.split(" And ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::And,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])],
                    }
                } else if lhs.contains("Or") {
                    let operands = lhs.split(" Or ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::Or,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])],
                    }
                } else if lhs.contains("Lshift") {
                    let operands = lhs.split(" Lshift ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::Lshift,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])],
                    }
                } else if lhs.contains("Rshift") {
                    let operands = lhs.split(" Rshift ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::Rshift,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])],
                    }
                } else {
                    Self {
                        op: OpCodes::Assign,
                        val: vec![get_u16(gates, lhs)],
                    }
                }
            }
        }
    }
}

fn collect_unsolved(query: &str) -> Vec<&str> {
    if query.contains("Not") {
        query
            .split("Not ")
            .skip(1)
            .filter(|l| l.parse::<u16>().is_err())
            .collect::<Vec<&str>>()
    } else if query.contains("And") {
        query
            .split(" And ")
            .filter(|l| l.parse::<u16>().is_err())
            .collect::<Vec<&str>>()
    } else if query.contains("Or") {
        query
            .split(" Or ")
            .filter(|l| l.parse::<u16>().is_err())
            .collect::<Vec<&str>>()
    } else if query.contains("Lshift") {
        query
            .split(" Lshift ")
            .filter(|l| l.parse::<u16>().is_err())
            .collect::<Vec<&str>>()
    } else if query.contains("Rshift") {
        query
            .split(" Rshift ")
            .filter(|l| l.parse::<u16>().is_err())
            .collect::<Vec<&str>>()
    } else if query.parse::<u16>().is_err() {
        vec![query]
    } else {
        vec![]
    }
}

fn solve(gates: &mut HashMap<String, MaybeInt>, id: &str) -> Result<()> {
    //query the hashmap (defensively, we'll be mutating later)
    let v = gates.get(id).expect(id).clone();
    match v {
        MaybeInt::Solved(_) => Ok(()),
        //entire query as string
        MaybeInt::Unsolved(query) => {
            //collect unsolved into vectors
            let to_solve = collect_unsolved(&query[..]);
            for s in to_solve.iter() {
                solve(gates, s).unwrap();
            }

            //at this point, queries should be solved (but not parsed)
            //so we can rebuild the query, DEFINITELY getting integers
            let v = gates.get(id).expect(id).clone();
            let op = Op::new(gates, &v);
            let v = match op.op {
                OpCodes::Assign => MaybeInt::Solved(op.val[0]),
                OpCodes::Rshift => MaybeInt::Solved(op.val[0] >> op.val[1]),
                OpCodes::Lshift => MaybeInt::Solved(op.val[0] << op.val[1]),
                OpCodes::And => MaybeInt::Solved(op.val[0] & op.val[1]),
                OpCodes::Or => MaybeInt::Solved(op.val[0] | op.val[1]),
                OpCodes::Not => MaybeInt::Solved(!op.val[0]),
            };
            let g = gates.get_mut(id).unwrap();
            *g = v;

            Ok(())
        }
    }
}

pub fn get_input() -> HashMap<String, MaybeInt> {
    let lines = include_str!("../../data/7.txt");
    let mut gates = HashMap::new();
    for l in lines.lines() {
        let cmd = l.split(" -> ").collect::<Vec<&str>>();
        let (lhs, id) = (cmd[0], cmd[1]);
        let mint = MaybeInt::new(lhs);
        gates.insert(id.to_string(), mint);
    }
    gates
}

pub fn part_1(gates: &HashMap<String, MaybeInt>) -> u16 {
    let mut gates = gates.clone();
    solve(&mut gates, "a").unwrap();
    match gates.get("a").unwrap() {
        MaybeInt::Solved(x) => *x,
        MaybeInt::Unsolved(_) => panic!("unable to solve"),
    }
}

pub fn part_2(gates: &HashMap<String, MaybeInt>) -> u16 {
    let mut first = gates.clone();
    let mut second = gates.clone();
    solve(&mut first, "a").unwrap();
    let a = first.get("a").unwrap();
    let b = second.get_mut("b").unwrap();
    if let MaybeInt::Solved(v) = a {
        *b = MaybeInt::Solved(*v);
    }
    solve(&mut second, "a").unwrap();
    match second.get("a").unwrap() {
        MaybeInt::Solved(v) => *v,
        MaybeInt::Unsolved(_) => panic!("unable to solve"),
    }
}
