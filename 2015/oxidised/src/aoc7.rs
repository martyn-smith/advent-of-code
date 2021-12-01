use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;

#[derive(Clone,Debug)]
pub enum MaybeInt {
    solved(u16),
    unsolved(String)
}

impl MaybeInt {
    fn new(left: &str) -> Self {
        if let Ok(x) = u16::from_str_radix(left, 10) {
            Self::solved(x)
        } else {
            Self::unsolved(left.to_string())
        }
    }
}

#[derive(Debug)]
enum OpCodes{
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    ASSIGN,
}

#[derive(Debug)]
struct Op {
    op : OpCodes,
    val : Vec<u16>
}

fn get_u16(gates: &HashMap<String, MaybeInt>, operand: &str) -> u16 {
    if let Ok(v) = u16::from_str_radix(operand, 10) {
        v
    } else if let Some(MaybeInt::solved(v)) = gates.get(operand) {
        *v
    } else {
        panic!()
    }
}

impl Op {
    fn new(gates: &HashMap<String, MaybeInt>, left: &MaybeInt) -> Self {
        match left {
            MaybeInt::solved(v) => {
            //I don't think we should ever be here...
                Self {
                    op: OpCodes::ASSIGN,
                    val: vec![*v]
                }
            },
            MaybeInt::unsolved(lhs) => {
                if lhs.contains("NOT") {
                    let operands = lhs.split("NOT ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::NOT,
                        val: vec![get_u16(gates, operands[1])]
                    }
                }
                else if lhs.contains("AND") {
                    let operands = lhs.split(" AND ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::AND,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])]
                    }
                }
                else if lhs.contains("OR") {
                    let operands = lhs.split(" OR ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::OR,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])]
                    }
                }
                else if lhs.contains("LSHIFT") {
                    let operands = lhs.split(" LSHIFT ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::LSHIFT,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])]
                    }
                }
                else if lhs.contains("RSHIFT") {
                    let operands = lhs.split(" RSHIFT ").collect::<Vec<&str>>();
                    Self {
                        op: OpCodes::RSHIFT,
                        val: vec![get_u16(gates, operands[0]), get_u16(gates, operands[1])]
                    }
                } else {
                    Self {
                        op: OpCodes::ASSIGN,
                        val: vec![get_u16(gates, lhs)]
                    }
                }
            }
        }
    }
}


fn collect_unsolved(query: & str) -> Vec<& str> {
    if query.contains("NOT") {
        query.split("NOT ").skip(1).filter(|l| !u16::from_str_radix(l, 10).is_ok()).collect::<Vec<&str>>()
    }
    else if query.contains("AND") {
        query.split(" AND ").filter(|l| !u16::from_str_radix(l, 10).is_ok()).collect::<Vec<&str>>()
    }
    else if query.contains("OR") {
        query.split(" OR ").filter(|l| !u16::from_str_radix(l, 10).is_ok()).collect::<Vec<&str>>()
    }
    else if query.contains("LSHIFT") {
        query.split(" LSHIFT ").filter(|l| !u16::from_str_radix(l, 10).is_ok()).collect::<Vec<&str>>()
    }
    else if query.contains("RSHIFT") {
        query.split(" RSHIFT ").filter(|l| !u16::from_str_radix(l, 10).is_ok()).collect::<Vec<&str>>()
    } else {
        if let Err(_) = u16::from_str_radix(query, 10) {
            vec![query]
        } else {
            vec![]
        }
    }
}

fn solve(gates: &mut HashMap<String, MaybeInt>, id: &str) -> Result<()> {
    //query the hashmap (defensively, we'll be mutating later)
    let v = gates.get(id).expect(id).clone();
    match v {
        MaybeInt::solved(u) => {
            Ok(())
        },
        //entire query as string
        MaybeInt::unsolved(query) => {
            //collect unsolved into vectors
            let to_solve = collect_unsolved(&query[..]);
            for s in to_solve.iter() {
                solve(gates, s);
            }

            //at this point, queries should be solved (but not parsed)
            //so we can rebuild the query, DEFINITELY getting integers
            let v = gates.get(id).expect(id).clone();
            let op = Op::new(gates, &v);
            let v = match op.op {
                OpCodes::ASSIGN => {

                    MaybeInt::solved(op.val[0])
                },
                OpCodes::RSHIFT => {
                    MaybeInt::solved(op.val[0] >> op.val[1])
                }
                OpCodes::LSHIFT => {
                    MaybeInt::solved(op.val[0] << op.val[1])
                }
                OpCodes::AND => {
                    MaybeInt::solved(op.val[0] & op.val[1])
                }
                OpCodes::OR => {
                    MaybeInt::solved(op.val[0] | op.val[1])
                }
                OpCodes::NOT => {
                    MaybeInt::solved(!op.val[0])
                }
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
    solve(&mut gates, "a");
    match gates.get("a").unwrap() {
        MaybeInt::solved(x) => *x,
        MaybeInt::unsolved(y) => panic!("unable to solve")
    }
}

pub fn part_2(gates: &HashMap<String, MaybeInt>) -> u16 {
    let mut first = gates.clone();
    let mut second = gates.clone();
    solve(&mut first, "a");
    let a = first.get("a").unwrap();
    let b = second.get_mut("b").unwrap();
    if let MaybeInt::solved(v) = a {
        *b = MaybeInt::solved(*v);
    }
    solve(&mut second, "a");
    match second.get("a").unwrap() {
        MaybeInt::solved(v) => *v,
        MaybeInt::unsolved(_) => panic!("unable to solve")
    }
}

