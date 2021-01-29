use super::intcode::Intcode;
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
    fn new(intcodes: &Intcode, phase: usize) -> Self {
        Amplifier {
            phase: phase,
            computer: intcodes.clone(),
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
    fn new(intcodes: &Intcode, phases: Vec<usize>) -> Self {
        AmpChain {
            amps: phases
                .iter()
                .map(|&p| Amplifier::new(intcodes, p))
                .collect(),
        }
    }

    fn run_open(&mut self, input: Option<isize>) -> Option<isize> {
        self.amps
            .iter_mut()
            .fold(input, |output, amp| amp.run(output))
    }

    fn run_closed(&mut self) -> usize {
        let mut output: Option<isize> = Some(0);
        loop {
            if let Some(o) = self.run_open(output)
            {
                output = Some(o);
            } else {
                break;
            }
        }
        output.unwrap() as usize
    }
}

pub fn get_input() -> Intcode {
    Intcode::load("../data/7.txt").unwrap()
}

pub fn part_1(intcodes: &Intcode) -> usize {
    let amp_count = 5;
    (0..amp_count)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(intcodes, phases);
            a.run_open(Some(0)).unwrap() as usize
        })
        .max()
        .unwrap()
}

pub fn part_2(intcodes: &Intcode) -> usize {
    let amp_count = 5;
    let offset = 5;
    (offset..amp_count + offset)
        .permutations(amp_count)
        .map(|phases| {
            let mut a = AmpChain::new(intcodes, phases);
            a.run_closed() as usize
        })
        .max()
        .unwrap()
}
