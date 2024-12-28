use std::convert::TryFrom;

pub enum Register {
    A,
    B
}

pub enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

struct CPU {
    a: usize,
    b: usize,
    ip: usize
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut s = line.split(' ');
        let first = s.next().unwrap();
        match first {
            "hlf" => {
                let reg = s.next().unwrap();
                match reg {
                    "a" => {
                        Ok(Self::Half(Register::A))
                    },

                    "b" => {
                        Ok(Self::Half(Register::B))
                    },
                    _ => {
                        Err("not a valid instruction")
                    }
                }
            },
            "tpl" => {
                let reg = s.next().unwrap();
                match reg {
                    "a" => {
                        Ok(Self::Triple(Register::A))
                    },

                    "b" => {
                        Ok(Self::Triple(Register::B))
                    },
                    _ => {
                        Err("not a valid instruction")
                    }
                }
            },
            "inc" => {
                let reg = s.next().unwrap();
                match reg {
                    "a" => {
                        Ok(Self::Increment(Register::A))
                    },

                    "b" => {
                        Ok(Self::Increment(Register::B))
                    },
                    _ => {
                        Err("not a valid instruction")
                    }
                }
            },
            "jmp" => {
                let offset = s.next().unwrap();
                if let Ok(o) = offset.parse::<i32>() {
                    Ok(Self::Jump(o))
                } else {
                    Err("invalid offset")
                }
            },
            "jie" => {
                let r = s.next().unwrap().split(',').next().unwrap();
                let reg = match r {
                    "a" => {
                        Ok(Register::A)
                    },

                    "b" => {
                        Ok(Register::B)
                    },
                    _ => {
                        Err("not a valid instruction")
                    }
                };
                let offset = s.next().unwrap().parse::<i32>();
                if let Ok(r) = reg {
                    if let Ok(o) = offset {
                        Ok(Self::JumpIfEven(r, o))
                    } else {
                        Err("not a valid instruction")
                    }
                }  else {
                    Err("not a valid instruction")
                }
            },
            "jio" => {
                let r = s.next().unwrap().split(',').next().unwrap();
                let reg = match r {
                    "a" => {
                        Ok(Register::A)
                    },

                    "b" => {
                        Ok(Register::B)
                    },
                    _ => {
                        Err("not a valid instruction")
                    }
                };
                let offset = s.next().unwrap().parse::<i32>();
                if let Ok(r) = reg {
                    if let Ok(o) = offset {
                        Ok(Self::JumpIfOne(r, o))
                    } else {
                        Err("not a valid instruction")
                    }
                }  else {
                    Err("not a valid instruction")
                }
            },
            _ => Err("not a valid instruction")
        }
    }
}

impl CPU {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            ip: 0
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        while let Some(i) = program.get(self.ip) {
            match i {
                Instruction::Half(r) => {
                    match r {
                      Register::A => {
                        self.a /= 2;
                      },
                      Register::B => {
                        self.b /= 2;
                      }
                    };
                    self.ip += 1;
                },
                Instruction::Triple(r) => {
                    match r {
                      Register::A => {
                        self.a *= 3;
                      },
                      Register::B => {
                        self.b *= 3;
                      }
                    };
                    self.ip += 1;
                },
                Instruction::Increment(r) => {
                    match r {
                      Register::A => {
                        self.a += 1;
                      },
                      Register::B => {
                        self.b += 1;
                      }
                    };
                    self.ip += 1;
                },
                Instruction::Jump(o) => {
                    self.ip = self.ip.checked_add_signed(*o as isize).unwrap_or(0);
                }
                Instruction::JumpIfEven(r, o) => {
                    let v = match r {
                        Register::A => {
                            self.a
                        },
                        Register::B => {
                            self.b
                        }
                    };
                    if v % 2 == 0 {
                        self.ip = self.ip.checked_add_signed(*o as isize).unwrap_or(0);
                    } else {
                        self.ip += 1;
                    }
                }
                Instruction::JumpIfOne(r, o) => {
                    let v = match r {
                        Register::A => {
                            self.a
                        },
                        Register::B => {
                            self.b
                        }
                    };
                    if v == 1 {
                        self.ip = self.ip.checked_add_signed(*o as isize).unwrap_or(0);
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
    }
}

pub fn get_input() -> Vec<Instruction> {
    include_str!("../../data/23.txt")
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect::<Vec<_>>()
}

pub fn part_1(program: &[Instruction]) -> usize {
    let mut cpu = CPU::new();
    cpu.run(program);
    cpu.b
}


pub fn part_2(program: &[Instruction]) -> usize {
    let mut cpu = CPU::new();
    cpu.a = 1;
    cpu.run(program);
    cpu.b
}
