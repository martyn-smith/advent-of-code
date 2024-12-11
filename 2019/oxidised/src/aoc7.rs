use crate::intcode::{Computer, FromStr, Program};
use anyhow::Result;
use itertools::Itertools;

struct Amplifier {
    phase: usize,
    program: Program,
    computer: Computer,
    first_run: bool,
}

struct AmpChain {
    amps: Vec<Amplifier>,
}

impl Amplifier {
    fn new(program: &Program, phase: usize) -> Self {
        Amplifier {
            phase,
            program: program.clone(),
            computer: Computer::new(),
            first_run: true,
        }
    }

    fn run(&mut self, input: Option<isize>) -> Option<isize> {
        let mut inputs = match self.first_run {
            true => {
                self.first_run = false;
                vec![input?, self.phase as isize]
            }
            false => vec![input?],
        };
        self.computer
            .next(&mut self.program, Some(&mut inputs))
            .unwrap()
    }
}

impl AmpChain {
    fn new(program: &Program, phases: Vec<usize>) -> Self {
        AmpChain {
            amps: phases.iter().map(|&p| Amplifier::new(program, p)).collect(),
        }
    }

    fn run_open(&mut self, input: Option<isize>) -> Option<isize> {
        self.amps
            .iter_mut()
            .fold(input, |output, amp| amp.run(output))
    }

    fn run_closed(&mut self) -> Option<isize> {
        let mut output: Option<isize> = Some(0);
        while let Some(o) = self.run_open(output) {
            output = Some(o);
        }
        output
    }
}

pub fn get_input() -> Program {
    Program::from_str(include_str!("../../data/7.txt")).unwrap()
}

pub fn part_1(program: &Program) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(program, phases);
            a.run_open(Some(0)).unwrap() as usize
        })
        .max()
        .unwrap()
}

pub fn part_2(program: &Program) -> usize {
    let amp_count = 5;
    let offset = 5;
    (offset..amp_count + offset)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(program, phases);
            a.run_closed().unwrap() as usize
        })
        .max()
        .unwrap()
}
