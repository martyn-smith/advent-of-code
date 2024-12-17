use anyhow::{bail, Context, Result};
pub use std::convert::From;
pub use std::ops::{Index, IndexMut};
pub use std::str::FromStr;

#[derive(Clone)]
pub struct Computer {
    ptr: usize,
    base: isize,
    halted: bool,
}

#[derive(Clone)]
pub struct Program {
    pub intcodes: Vec<isize>,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            base: 0,
            halted: false,
        }
    }

    fn add(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 1 adds two integers, storing the result in the third parameter.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = a + b;
        self.ptr += 4;
        None
    }

    fn mul(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 2 multiplies two integers, storing the result in the third parameter.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = a * b;
        self.ptr += 4;
        None
    }

    fn input(&mut self, program: &mut Program, input: isize) -> Option<isize> {
        // Opcode 3 writes an input to the first parameter.
        let modes = self.get_modes(program, self.ptr);
        let i = self.get_pointer(program, self.ptr + 1, modes[0]);
        program[i] = input;
        self.ptr += 2;
        None
    }

    fn output(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 4 outputs from the first parameter.
        let modes = self.get_modes(program, self.ptr);
        let output = self.get_value(program, self.ptr + 1, modes[0]);
        self.ptr += 2;
        Some(output)
    }

    fn jt(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 5 is jump-if-true: if the first parameter is non-zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        if a != 0 {
            self.ptr = b as usize;
        } else {
            self.ptr += 3;
        }
        None
    }

    fn jf(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 6 is jump-if-false: if the first parameter is zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        if a == 0 {
            self.ptr = b as usize;
        } else {
            self.ptr += 3;
        }
        None
    }

    fn lt(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 7 is less than: if the first parameter is less than the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = if a < b { 1 } else { 0 };
        self.ptr += 4;
        None
    }

    fn eq(&mut self, program: &mut Program) -> Option<isize> {
        // Opcode 8 is equals: if the first parameter is equal to the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = if a == b { 1 } else { 0 };
        self.ptr += 4;
        None
    }

    fn rb(&mut self, program: &mut Program) -> Option<isize> {
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        self.base += a;
        self.ptr += 2;
        None
    }

    fn halt(&mut self) -> Option<isize> {
        // Opcode 99 halts the CPU.
        self.halted = true;
        None
    }

    fn get_modes(&self, program: &Program, pos: usize) -> [isize; 3] {
        let mode_args = program.intcodes[pos] / 100;
        let first = mode_args % 10;
        let second = (mode_args / 10) % 10;
        let third = (mode_args / 100) % 10;
        [first, second, third]
    }

    fn get_value(&self, program: &Program, pos: usize, mode_arg: isize) -> isize {
        let ptr = match mode_arg {
            //position mode
            0 => program.intcodes[pos] as usize,
            //immediate mode
            1 => pos,
            //relative mode
            2 => (self.base + program.intcodes[pos]) as usize,
            _ => {
                panic!();
            }
        };
        if ptr < program.intcodes.len() {
            program.intcodes[ptr]
        } else {
            0
        }
    }

    fn get_pointer(&self, program: &Program, pos: usize, mode_arg: isize) -> usize {
        match mode_arg {
            //position mode
            0 => program.intcodes[pos] as usize,
            //relative mode
            2 => (self.base + program.intcodes[pos]) as usize,
            _ => {
                panic!();
            }
        }
    }

    fn step(&mut self, program: &mut Program, inputs: &mut Vec<isize>) -> Result<Option<isize>> {
        let result = match program.intcodes[self.ptr] % 100 {
            1 => self.add(program),
            2 => self.mul(program),
            3 => self.input(program, inputs.pop().context("ran out of inputs")?),
            4 => self.output(program),
            5 => self.jt(program),
            6 => self.jf(program),
            7 => self.lt(program),
            8 => self.eq(program),
            9 => self.rb(program),
            99 => self.halt(),
            _ => {
                bail!("invalid opcode at {}", self.ptr);
            }
        };
        Ok(result)
    }

    //runs to first call to output
    pub fn next(
        &mut self,
        program: &mut Program,
        inputs: Option<&mut Vec<isize>>,
    ) -> Result<Option<isize>> {
        let mut empty = vec![];
        let inputs = inputs.unwrap_or(&mut empty);
        while !self.halted {
            if let Some(x) = self.step(program, inputs)? {
                return Ok(Some(x));
            }
        }
        Ok(None)
    }

    //run to halt
    pub fn run(
        &mut self,
        program: &mut Program,
        inputs: Option<&mut Vec<isize>>,
    ) -> Result<Vec<isize>> {
        let mut empty = vec![];
        let inputs = inputs.unwrap_or(&mut empty);
        let mut outputs: Vec<isize> = vec![];
        while !self.halted {
            if let Some(x) = self.step(program, inputs)? {
                outputs.push(x);
            }
        }
        Ok(outputs)
    }
}

impl From<&[isize]> for Program {
    fn from(intcodes: &[isize]) -> Self {
        Self {
            intcodes: intcodes.to_owned(),
        }
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let intcodes = s
            .trim()
            .split(',')
            .map(|l| l.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { intcodes })
    }
}

impl Index<usize> for Program {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intcodes[index]
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.intcodes.len() {
            self.intcodes.resize(index + 1, 0);
        }
        &mut self.intcodes[index]
    }
}

pub fn solve(mut program: Program, inputs: Option<Vec<isize>>) -> Result<Vec<isize>> {
    // Convenience wrapper for the many problems that require we simply create the program,
    // then run it.
    let empty = vec![];
    let mut inputs = inputs.unwrap_or(empty);
    let mut computer = Computer::new();
    computer.run(&mut program, Some(&mut inputs))
}

// convenience for problems that require ASCII output.
pub fn ascii(mut program: Program, inputs: Option<Vec<isize>>) -> Result<String> {
    let empty = vec![];
    let mut inputs = inputs.unwrap_or(empty);
    let mut computer = Computer::new();
    Ok(computer
        .run(&mut program, Some(&mut inputs))?
        .into_iter()
        .map(|c| char::from(c as u8))
        .collect::<String>())
}
