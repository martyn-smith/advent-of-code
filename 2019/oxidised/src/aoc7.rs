use super::intcode::Intcode;
use itertools::Itertools;
use std::fs;

struct Amplifier {
    phase: usize,
    computer: Intcode,
    first_run: bool,
}

struct AmpChain {
    amps: Vec<Amplifier>,
}

impl Amplifier {
    fn new(intcodes: &Vec<isize>, phase: usize) -> Self {
        Amplifier {
            phase: phase,
            computer: Intcode::new(intcodes),
            first_run: true,
        }
    }

    fn run(&mut self, input: Option<isize>) -> Option<isize> {
        if let Some(input) = input {
            let inputs = match self.first_run {
                true => {
                    self.first_run = false;
                    vec![input as isize, self.phase as isize]
                }
                false => vec![input as isize],
            };
            self.computer.step(inputs)
        } else {
            None
        }
    }
}

impl AmpChain {
    fn new(intcodes: &Vec<isize>, phases: Vec<usize>) -> Self {
        AmpChain {
            amps: phases
                .iter()
                .map(|&p| Amplifier::new(intcodes, p))
                .collect(),
        }
    }

    fn run_open(&mut self) -> usize {
        self.amps
            .iter_mut()
            .fold(Some(0), |output, amp| amp.run(output))
            .unwrap() as usize
    }

    fn run_closed(&mut self) -> usize {
        let mut output: Option<isize> = Some(0);
        loop {
            if let Some(o) = self
                .amps
                .iter_mut()
                .fold(output, |output, amp| amp.run(output))
            {
                output = Some(o);
            } else {
                break;
            }
        }
        output.unwrap() as usize
    }
}

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/7.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(intcodes, phases);
            a.run_open()
        })
        .max()
        .unwrap()
}

pub fn part_2(intcodes: &Vec<isize>) -> usize {
    let amp_count = 5;
    let offset = 5;
    let test = vec![9, 8, 7, 6, 5];
    let mut a = AmpChain::new(intcodes, test);
    println!("{}", a.run_closed());
    (offset..amp_count + offset)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(intcodes, phases);
            a.run_closed()
        })
        .max()
        .unwrap()
}
