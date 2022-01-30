use anyhow::{bail, Result};
use std::fs;

pub type Program = Vec<isize>;

pub fn from_str(s: &'static str) -> Program {
    s
    .trim()
    .split(',')
    .map(|l| l.parse::<isize>().unwrap())
    .collect::<Vec<isize>>()
}

struct Pause {
    advance: usize,
    value: isize
}

enum OpResult {
    Advance(usize),
    Pause(usize, isize),
    NoOp,
    Halt
}

pub struct Intcode<'a>  {
    pub program: &'a mut Program,
    pub inputs: Program,
    ptr: usize,
    base: isize,
}

impl<'a> Iterator for Intcode<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        'l: loop {
            let r = match self.program[self.ptr] % 100 {
                1 => self.add(),
                2 => self.mult(),
                3 => self.input(),
                4 => self.output(),
                5 => self.jt(),
                6 => self.jf(),
                7 => self.lt(),
                8 => self.eq(),
                9 => self.rb(),
                99 => OpResult::Halt,
                _ => {
                    panic!("invalid opcode at {}", self.ptr);
                }
            };
            match r {
                OpResult::Advance(i) => {
                    self.ptr += i;
                },
                OpResult::Pause(i, o) => {
                    self.ptr += i;
                    break 'l Some(o);
                },
                OpResult::NoOp => {
                },
                OpResult::Halt => {
                    break 'l None;
                }
            }
        }
    }
}

impl<'a>  Intcode<'a> {
    fn add(&mut self) -> OpResult {
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w >= self.program.len() {
            self.program.resize(w + 1, 0);
        }
        self.program[w] = a + b;
        OpResult::Advance(4)
    }

    fn mult(&mut self) -> OpResult {
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w >= self.program.len() {
            self.program.resize(w + 1, 0);
        }
        self.program[w] = a * b;
        OpResult::Advance(4)
    }

    fn input(&mut self) -> OpResult {
        let mode_arg = self.program[self.ptr] / 100;
        let w = self.get_pointer(self.ptr + 1, mode_arg % 10) as usize;
        let write_val = self.inputs.pop().unwrap();
        if w >= self.program.len() {
            self.program.resize(w + 1, 0);
        }
        self.program[w] = write_val;
        OpResult::Advance(2)
    }

    fn output(&self) -> OpResult {
        let mode_arg = self.program[self.ptr] / 100;
        let read_pos = self.get_value(self.ptr + 1, mode_arg);
        OpResult::Pause(2, read_pos)
    }

    fn jt(&mut self) -> OpResult {
        //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a != 0 {
            self.ptr = b as usize;
            OpResult::NoOp
        } else {
            OpResult::Advance(3)
        }
    }

    fn jf(&mut self) -> OpResult {
        // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a == 0 {
            self.ptr = b as usize;
            OpResult::NoOp
        } else {
            OpResult::Advance(3)
        }
    }

    fn lt(&mut self) -> OpResult {
        // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w as usize >= self.program.len() {
            self.program.resize(w + 1, 0);
        }
        self.program[w] = if a < b { 1 } else { 0 };
        OpResult::Advance(4)
    }

    fn eq(&mut self) -> OpResult {
        // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let mode_args = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w as usize >= self.program.len() {
            self.program.resize(w + 1, 0);
        }
        self.program[w] = if a == b { 1 } else { 0 };
        OpResult::Advance(4)
    }

    fn rb(&mut self) -> OpResult {
        let mode_arg = self.program[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_arg);
        self.base += a;
        OpResult::Advance(2)
    }

    fn get_value(&self, pos: usize, mode_arg: isize) -> isize {
        let ptr = match mode_arg {
            //position mode
            0 => self.program[pos] as usize,
            //immediate mode
            1 => pos,
            //relative mode
            2 => (self.base + self.program[pos]) as usize,
            _ => {
                panic!();
            }
        };
        if ptr < self.program.len() {
            self.program[ptr]
        } else {
            0
        }
    }

    fn get_pointer(&self, pos: usize, mode_arg: isize) -> isize {
        match mode_arg {
            //position mode
            0 => self.program[pos],
            //relative mode
            2 => self.base + self.program[pos],
            _ => {
                panic!();
            }
        }
    }

    pub fn new(program: &'a mut Program) -> Self {
        Self {
            program: program,
            inputs: vec![],
            ptr: 0,
            base: 0,
        }
    }

    pub fn run(&mut self, inputs: Program) -> Program {
        self.inputs = inputs;
        self.collect::<Program>()
    }

    pub fn ascii(&mut self, inputs: Vec<isize>) -> Result<String> {
        Ok(String::from_utf8(
            self.run(inputs).iter().map(|&c| c as u8).collect(),
        )?)
    }
}
