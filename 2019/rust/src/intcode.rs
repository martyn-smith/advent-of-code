use anyhow::{bail, Context, Result};
pub use std::convert::From;
pub use std::ops::{Index, IndexMut};
pub use std::str::FromStr;

#[derive(Clone)]
pub struct Computer {
    ptr: usize,
    base: isize,
    halted: bool,
    inputs: Vec<isize>,
    outputs: Vec<isize>,
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
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn add(&mut self, program: &mut Program) {
        // Opcode 1 adds two integers, storing the result in the third parameter.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = a + b;
        self.ptr += 4;
    }

    fn mul(&mut self, program: &mut Program) {
        // Opcode 2 multiplies two integers, storing the result in the third parameter.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = a * b;
        self.ptr += 4;
    }

    fn input(&mut self, program: &mut Program) -> Result<()> {
        // Opcode 3 writes an input to the first parameter.
        let input = self
            .inputs
            .pop()
            .context("no inputs available to Intcode")?;
        let modes = self.get_modes(program, self.ptr);
        let i = self.get_pointer(program, self.ptr + 1, modes[0]);
        program[i] = input;
        self.ptr += 2;
        Ok(())
    }

    fn output(&mut self, program: &mut Program) {
        // Opcode 4 outputs from the first parameter.
        let modes = self.get_modes(program, self.ptr);
        let output = self.get_value(program, self.ptr + 1, modes[0]);
        self.outputs.push(output);
        self.ptr += 2;
    }

    fn jt(&mut self, program: &mut Program) {
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
    }

    fn jf(&mut self, program: &mut Program) {
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
    }

    fn lt(&mut self, program: &mut Program) {
        // Opcode 7 is less than: if the first parameter is less than the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = if a < b { 1 } else { 0 };
        self.ptr += 4;
    }

    fn eq(&mut self, program: &mut Program) {
        // Opcode 8 is equals: if the first parameter is equal to the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        let b = self.get_value(program, self.ptr + 2, modes[1]);
        let i = self.get_pointer(program, self.ptr + 3, modes[2]);
        program[i] = if a == b { 1 } else { 0 };
        self.ptr += 4;
    }

    fn rb(&mut self, program: &mut Program) {
        let modes = self.get_modes(program, self.ptr);
        let a = self.get_value(program, self.ptr + 1, modes[0]);
        self.base += a;
        self.ptr += 2;
    }

    fn halt(&mut self) {
        // Opcode 99 halts the CPU.
        self.halted = true;
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

    fn step(&mut self, program: &mut Program) -> Result<()> {
        match program.intcodes[self.ptr] % 100 {
            1 => {
                self.add(program);
                Ok(())
            }
            2 => {
                self.mul(program);
                Ok(())
            }
            3 => self.input(program),
            4 => {
                self.output(program);
                Ok(())
            }
            5 => {
                self.jt(program);
                Ok(())
            }
            6 => {
                self.jf(program);
                Ok(())
            }
            7 => {
                self.lt(program);
                Ok(())
            }
            8 => {
                self.eq(program);
                Ok(())
            }
            9 => {
                self.rb(program);
                Ok(())
            }
            99 => {
                self.halt();
                Ok(())
            }
            _ => {
                bail!("invalid opcode at {}", self.ptr);
            }
        }
    }

    //runs to first call to output
    pub fn next(
        &mut self,
        program: &mut Program,
        inputs: Option<&mut Vec<isize>>,
    ) -> Result<Option<isize>> {
        if let Some(inp) = inputs {
            self.inputs.append(inp);
        }
        while !self.halted && self.outputs.is_empty() {
            self.step(program)?;
        }
        Ok(self.outputs.pop())
    }

    //run to halt
    pub fn run(
        &mut self,
        program: &mut Program,
        inputs: Option<&mut Vec<isize>>,
    ) -> Result<Vec<isize>> {
        if let Some(inp) = inputs {
            self.inputs.append(inp);
        }
        while !self.halted {
            self.step(program)?;
        }
        Ok(self.outputs.to_owned())
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
