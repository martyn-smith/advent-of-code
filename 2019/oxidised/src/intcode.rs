use anyhow::{bail, Result};
use std::fs;

#[derive(Clone)]
pub struct Intcode {
    pub intcodes: Vec<isize>,
    ptr: usize,
    base: isize,
}

enum State {
    Continue(usize),
    Pause(usize),
    Halt,
}

impl Intcode {
    fn add(&mut self) -> usize {
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w >= self.intcodes.len() {
            self.intcodes.resize(w + 1, 0);
        }
        self.intcodes[w] = a + b;
        4
    }

    fn mult(&mut self) -> usize {
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w >= self.intcodes.len() {
            self.intcodes.resize(w + 1, 0);
        }
        self.intcodes[w] = a * b;
        4
    }

    fn input(&mut self, input: &mut Vec<isize>) -> usize {
        let mode_arg = self.intcodes[self.ptr] / 100;
        let w = self.get_pointer(self.ptr + 1, mode_arg % 10) as usize;
        let write_val = input.pop().unwrap();
        if w >= self.intcodes.len() {
            self.intcodes.resize(w + 1, 0);
        }
        self.intcodes[w] = write_val;
        2
    }

    fn output(&self, outputs: &mut Vec<isize>) -> usize {
        let mode_arg = self.intcodes[self.ptr] / 100;
        let read_pos = self.get_value(self.ptr + 1, mode_arg);
        outputs.push(read_pos);
        2
    }

    fn jt(&mut self) -> usize {
        //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a != 0 {
            self.ptr = b as usize;
            0
        } else {
            3
        }
    }

    fn jf(&mut self) -> usize {
        // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a == 0 {
            self.ptr = b as usize;
            0
        } else {
            3
        }
    }

    fn lt(&mut self) -> usize {
        // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w as usize >= self.intcodes.len() {
            self.intcodes.resize(w + 1, 0);
        }
        self.intcodes[w] = if a < b { 1 } else { 0 };
        4
    }

    fn eq(&mut self) -> usize {
        // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        let w = self.get_pointer(self.ptr + 3, (mode_args / 100) % 10) as usize;
        if w as usize >= self.intcodes.len() {
            self.intcodes.resize(w + 1, 0);
        }
        self.intcodes[w] = if a == b { 1 } else { 0 };
        4
    }

    fn rb(&mut self) -> usize {
        let mode_arg = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_arg);
        self.base += a;
        2
    }

    fn get_value(&self, pos: usize, mode_arg: isize) -> isize {
        let ptr = match mode_arg {
            //position mode
            0 => self.intcodes[pos] as usize,
            //immediate mode
            1 => pos,
            //relative mode
            2 => (self.base + self.intcodes[pos]) as usize,
            _ => {
                panic!();
            }
        };
        if ptr < self.intcodes.len() {
            self.intcodes[ptr]
        } else {
            0
        }
    }

    fn get_pointer(&self, pos: usize, mode_arg: isize) -> isize {
        match mode_arg {
            //position mode
            0 => self.intcodes[pos],
            //relative mode
            2 => self.base + self.intcodes[pos],
            _ => {
                panic!();
            }
        }
    }

    // plan to discontinue, or at least redoc, these
    pub fn new(intcodes: &Vec<isize>) -> Self {
        Intcode {
            intcodes: intcodes.clone(),
            ptr: 0,
            base: 0,
        }
    }

    pub fn load(filename: &str) -> Result<Self> {
        let intcodes = fs::read_to_string(filename)?
            .trim()
            .split(',')
            .map(|l| l.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Ok(Intcode {
            intcodes,
            ptr: 0,
            base: 0,
        })
    }
    //
    pub fn from_str(s: &str) -> Result<Self> {
        let intcodes = s
            .trim()
            .split(',')
            .map(|l| l.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Ok(Self {
            intcodes,
            ptr: 0,
            base: 0,
        })
    }

    pub fn from_vec(intcodes: &Vec<isize>) -> Result<Self> {
        Ok(Self {
            intcodes: intcodes.clone(),
            ptr: 0,
            base: 0,
        })
    }

    pub fn ascii(&mut self, inputs: Vec<isize>) -> Result<String> {
        Ok(String::from_utf8(
            self.run(inputs)?.iter().map(|&c| c as u8).collect(),
        )?)
    }

    //run to halt
    //TODO: or lack of inputs
    pub fn run(&mut self, mut inputs: Vec<isize>) -> Result<Vec<isize>> {
        let mut outputs: Vec<isize> = vec![];
        'a: loop {
            //dbg!(self.ptr, self.intcodes[self.ptr] % 100);
            let adv = match self.intcodes[self.ptr] % 100 {
                1 => State::Continue(self.add()),
                2 => State::Continue(self.mult()),
                3 => State::Continue(self.input(&mut inputs)),
                /*
                 * 3 => if let Some(i) = inputs.pop() { State::Continue(i)} else { State::Pause }
                 */
                4 => State::Continue(self.output(&mut outputs)),
                5 => State::Continue(self.jt()),
                6 => State::Continue(self.jf()),
                7 => State::Continue(self.lt()),
                8 => State::Continue(self.eq()),
                9 => State::Continue(self.rb()),
                99 => State::Halt,
                _ => {
                    bail!("invalid opcode at {}", self.ptr);
                }
            };
            if let State::Continue(i) = adv {
                self.ptr += i;
            } else {
                break 'a;
            }
        }
        Ok(outputs)
    }

    pub fn step(&mut self, mut inputs: Vec<isize>) -> Option<isize> {
        let mut outputs: Vec<isize> = vec![];
        loop {
            //dbg!(self.ptr, self.intcodes[self.ptr] % 100);
            let adv = match self.intcodes[self.ptr] % 100 {
                1 => State::Continue(self.add()),
                2 => State::Continue(self.mult()),
                3 => State::Continue(self.input(&mut inputs)),
                4 => State::Pause(self.output(&mut outputs)),
                5 => State::Continue(self.jt()),
                6 => State::Continue(self.jf()),
                7 => State::Continue(self.lt()),
                8 => State::Continue(self.eq()),
                9 => State::Continue(self.rb()),
                99 => State::Halt,
                _ => {
                    panic!("invalid opcode at {}", self.ptr);
                }
            };
            match adv {
                State::Continue(i) => {
                    self.ptr += i;
                }
                State::Pause(i) => {
                    self.ptr += i;
                    return Some(*outputs.last().unwrap());
                }
                State::Halt => {
                    return None;
                }
            }
        }
    }
}
