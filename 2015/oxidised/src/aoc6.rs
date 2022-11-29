use regex::{Match, Regex};

enum Action {
    Off,
    On,
    Toggle,
}

pub struct Instruction {
    cmd: Action,
    start: (usize, usize),
    end: (usize, usize),
}

impl Instruction {
    fn new(l: &str) -> Self {
        let get_nums = Regex::new(r"\d*,\d*").unwrap();
        let idxs: Vec<Match> = get_nums.find_iter(l).collect();
        Instruction {
            cmd: match &l[..idxs[0].start()] {
                "turn on " => Action::On,
                "turn off " => Action::Off,
                "toggle " => Action::Toggle,
                _ => unreachable!(),
            },
            start: {
                let x: Vec<&str> = l[idxs[0].start()..idxs[0].end()].split(',').collect();
                (
                    x[0].parse::<usize>().unwrap(),
                    x[1].parse::<usize>().unwrap(),
                )
            },
            end: {
                let x: Vec<&str> = l[idxs[1].start()..idxs[1].end()].split(',').collect();
                (
                    x[0].parse::<usize>().unwrap(),
                    x[1].parse::<usize>().unwrap(),
                )
            },
        }
    }
}

pub fn get_input() -> Vec<Instruction> {
    include_str!("../../data/6.txt")
        .lines()
        .map(Instruction::new)
        .collect()
}

pub fn part_1(instructions: &[Instruction]) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    for instr in instructions.iter() {
        for i in instr.start.0..=instr.end.0 {
            for j in instr.start.1..=instr.end.1 {
                match instr.cmd {
                    Action::On => {
                        lights[i][j] = true;
                    }
                    Action::Off => {
                        lights[i][j] = false;
                    }
                    Action::Toggle => {
                        lights[i][j] = !lights[i][j];
                    }
                }
            }
        }
    }
    lights
        .iter()
        .map(|row| row.iter().filter(|&&l| l).count())
        .sum()
}

pub fn part_2(instructions: &[Instruction]) -> usize {
    let mut lights = vec![vec![0usize; 1000]; 1000];
    for instr in instructions.iter() {
        for i in instr.start.0..=instr.end.0 {
            for j in instr.start.1..=instr.end.1 {
                match instr.cmd {
                    Action::On => {
                        lights[i][j] += 1;
                    }
                    Action::Off => {
                        lights[i][j] = lights[i][j].saturating_sub(1);
                    }
                    Action::Toggle => {
                        lights[i][j] += 2;
                    }
                }
            }
        }
    }
    lights.iter().map(|row| row.iter().sum::<usize>()).sum()
}
