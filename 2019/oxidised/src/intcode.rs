pub struct Intcode {
    pub intcodes: Vec<isize>,
    ptr: usize,
}

impl Intcode {
    fn add(&mut self) -> usize {
        let write_pos = self.intcodes[self.ptr + 3] as usize;
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        self.intcodes[write_pos] = a + b;
        4
    }

    fn mult(&mut self) -> usize {
        let write_pos = self.intcodes[self.ptr + 3] as usize;
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        self.intcodes[write_pos] = a * b;
        4
    }

    fn input(&mut self, input: &mut Vec<isize>) -> usize {
        let write_pos = self.intcodes[self.ptr + 1] as usize;
        let write_val = input.pop().unwrap();
        self.intcodes[write_pos] = write_val;
        2
    }

    fn output(&self, outputs: &mut Vec<isize>) -> usize {
        let mode_arg = self.intcodes[self.ptr] / 100;
        let read_pos = self.get_value(self.ptr + 1, mode_arg);
        outputs.push(read_pos);
        2
    }

    fn jt(&self) -> usize {
        //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a != 0 {
            b as usize - self.ptr
        } else {
            3
        }
    }

    fn jf(&self) -> usize {
        // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        if a == 0 {
            b as usize - self.ptr
        } else {
            3
        }
    }

    fn lt(&mut self) -> usize {
        // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let write_pos = self.intcodes[self.ptr + 3] as usize;
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        self.intcodes[write_pos] = if a < b { 1 } else { 0 };
        4
    }

    fn eq(&mut self) -> usize {
        // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        let write_pos = self.intcodes[self.ptr + 3] as usize;
        let mode_args = self.intcodes[self.ptr] / 100;
        let a = self.get_value(self.ptr + 1, mode_args % 10);
        let b = self.get_value(self.ptr + 2, (mode_args / 10) % 10);
        self.intcodes[write_pos] = if a == b { 1 } else { 0 };
        4
    }

    fn get_value(&self, pos: usize, mode_arg: isize) -> isize {
        match mode_arg {
            //position mode
            0 => {
                let ptr = self.intcodes[pos] as usize;
                self.intcodes[ptr]
            }
            1 => self.intcodes[pos],
            _ => {
                panic!();
            }
        }
    }

    pub fn new(intcodes: &Vec<isize>) -> Self {
        Intcode {
            intcodes: intcodes.clone(),
            ptr: 0,
        }
    }

    pub fn run(&mut self, mut inputs: Vec<isize>) -> Result<Vec<isize>, isize> {
        let mut outputs: Vec<isize> = vec![];
        'a: loop {
            let adv = match self.intcodes[self.ptr] % 100 {
                1 => self.add(),
                2 => self.mult(),
                3 => self.input(&mut inputs),
                4 => self.output(&mut outputs),
                5 => self.jt(),
                6 => self.jf(),
                7 => self.lt(),
                8 => self.eq(),
                99 => {
                    break 'a;
                }
                _ => {
                    return Err(-1);
                }
            };
            self.ptr += adv;
        }
        Ok(outputs)
    }
}
