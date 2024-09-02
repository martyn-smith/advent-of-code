use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum MaybeInt {
    Value(isize),
    Register(char),
}

#[derive(Clone, Debug)]
pub enum Op {
    Inp(char),
    Mul(char, MaybeInt),
    Add(char, MaybeInt),
    Mod(char, MaybeInt),
    Div(char, MaybeInt),
    Eql(char, MaybeInt),
}

#[allow(clippy::upper_case_acronyms)]
pub struct ALU {
    instructions: VecDeque<Op>,
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl MaybeInt {
    fn new(s: &str) -> Self {
        if let Ok(n) = s.parse::<isize>() {
            Self::Value(n)
        } else {
            Self::Register(s.chars().next().unwrap())
        }
    }
}

impl Op {
    pub fn new(l: &str) -> Self {
        let mut s = l.split(' ');
        let op = s.next().unwrap();
        let first = s.next().unwrap();
        let second = s.next();
        match op {
            "inp" => Self::Inp(first.chars().next().unwrap()),
            "mul" => Self::Mul(
                first.chars().next().unwrap(),
                MaybeInt::new(second.unwrap()),
            ),
            "add" => Self::Add(
                first.chars().next().unwrap(),
                MaybeInt::new(second.unwrap()),
            ),
            "mod" => Self::Mod(
                first.chars().next().unwrap(),
                MaybeInt::new(second.unwrap()),
            ),
            "div" => Self::Div(
                first.chars().next().unwrap(),
                MaybeInt::new(second.unwrap()),
            ),
            "eql" => Self::Eql(
                first.chars().next().unwrap(),
                MaybeInt::new(second.unwrap()),
            ),
            _ => panic!("couldn't parse input: {}", l),
        }
    }
}

impl ALU {
    fn get_register(&mut self, i: char) -> &mut isize {
        match i {
            'w' => &mut self.w,
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => unreachable!(),
        }
    }

    fn get_value(&self, b: MaybeInt) -> isize {
        match b {
            MaybeInt::Value(i) => i,
            MaybeInt::Register(c) => match c {
                'w' => self.w,
                'x' => self.x,
                'y' => self.y,
                'z' => self.z,
                _ => unreachable!(),
            },
        }
    }

    pub fn new(instructions: &[Op]) -> Self {
        let instructions = VecDeque::from(instructions.to_vec());
        Self {
            instructions,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn run(&mut self, mut input: Vec<u8>) -> isize {
        while let Some(ins) = self.instructions.pop_front() {
            match ins {
                Op::Inp(r) => {
                    let i = input.pop().unwrap();
                    let register = self.get_register(r);
                    *register = i as isize;
                }
                Op::Mul(a, b) => {
                    let i = self.get_value(b);
                    let register = self.get_register(a);
                    *register *= i;
                }
                Op::Add(a, b) => {
                    let i = self.get_value(b);
                    let register = self.get_register(a);
                    *register += i;
                }
                Op::Mod(a, b) => {
                    let i = self.get_value(b);
                    let register = self.get_register(a);
                    *register %= i;
                }
                Op::Div(a, b) => {
                    let i = self.get_value(b);
                    let register = self.get_register(a);
                    *register /= i;
                }
                Op::Eql(a, b) => {
                    let i = self.get_value(b);
                    let register = self.get_register(a);
                    *register = if *register == i { 1 } else { 0 };
                }
            }
        }
        self.z
    }
}
