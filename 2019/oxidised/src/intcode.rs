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
    //
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
        //let w = self.intcodes[self.ptr + 1] as usize;
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

    pub fn new(intcodes: &Vec<isize>) -> Self {
        Intcode {
            intcodes: intcodes.clone(),
            ptr: 0,
            base: 0,
        }
    }

    pub fn load(filename: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let intcodes = fs::read_to_string(filename)?
            .trim()
            .split(',')
            .map(|l| l.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Ok(Intcode {
            intcodes: intcodes,
            ptr: 0,
            base: 0,
        })
    }

    pub fn run(&mut self, mut inputs: Vec<isize>) -> Result<Vec<isize>, isize> {
        let mut outputs: Vec<isize> = vec![];
        'a: loop {
            //dbg!(self.ptr, self.intcodes[self.ptr] % 100);
            let adv = match self.intcodes[self.ptr] % 100 {
                1 => self.add(),
                2 => self.mult(),
                3 => self.input(&mut inputs),
                4 => self.output(&mut outputs),
                5 => self.jt(),
                6 => self.jf(),
                7 => self.lt(),
                8 => self.eq(),
                9 => self.rb(),
                99 => {
                    break 'a;
                }
                _ => {
                    println!("invalid opcode at {}", self.ptr);
                    return Err(-1);
                }
            };
            self.ptr += adv;
        }
        Ok(outputs)
    }

    pub fn step(&mut self, mut inputs: Vec<isize>) -> Option<isize> {
        let mut outputs: Vec<isize> = vec![];
        loop {
            //dbg!(self.ptr, self.intcodes[self.ptr] % 100);
            let adv = match self.intcodes[self.ptr] % 100 {
                1 => self.add(),
                2 => self.mult(),
                3 => self.input(&mut inputs),
                4 => {
                    self.ptr += self.output(&mut outputs);
                    return Some(*outputs.last().unwrap());
                }
                5 => self.jt(),
                6 => self.jf(),
                7 => self.lt(),
                8 => self.eq(),
                9 => self.rb(),
                99 => {
                    return None;
                }
                _ => {
                    panic!("invalid opcode at {}", self.ptr);
                }
            };
            self.ptr += adv;
        }
    }
}
