use super::intcode::{FromStr, Intcode};
use itertools::Itertools;

struct Amplifier {
    phase: usize,
    computer: Intcode,
    first_run: bool,
}

struct AmpChain {
    amps: Vec<Amplifier>,
}

impl Amplifier {
    fn new(computer: &Intcode, phase: usize) -> Self {
        Amplifier {
            phase,
            computer: computer.clone(),
            first_run: true,
        }
    }

    fn run(&mut self, input: Option<isize>) -> Option<isize> {
        let inputs = match self.first_run {
            true => {
                self.first_run = false;
                vec![input?, self.phase as isize]
            }
            false => vec![input?],
        };
        self.computer.step(inputs)
    }
}

impl AmpChain {
    fn new(computer: &Intcode, phases: Vec<usize>) -> Self {
        AmpChain {
            amps: phases
                .iter()
                .map(|&p| Amplifier::new(computer, p))
                .collect(),
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

pub fn get_input() -> Intcode {
    Intcode::from_str(include_str!("../../data/7.txt")).unwrap()
}

pub fn part_1(computer: &Intcode) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(computer, phases);
            a.run_open(Some(0)).unwrap() as usize
        })
        .max()
        .unwrap()
}

pub fn part_2(computer: &Intcode) -> usize {
    let amp_count = 5;
    let offset = 5;
    (offset..amp_count + offset)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(computer, phases);
            a.run_closed().unwrap() as usize
        })
        .max()
        .unwrap()
}
