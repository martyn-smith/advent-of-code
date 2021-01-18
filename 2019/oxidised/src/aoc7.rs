use super::intcode::Intcode;
use itertools::Itertools;
use std::fs;

fn run_amps(intcodes: &Vec<isize>, phase: Vec<usize>) -> usize {
    //each intcode run will require two inputs: phase setting (i'th element of p),
    // plus output of last intcode run

    phase.iter().fold(0isize, |output, &p| {
        let mut computer = Intcode::new(intcodes);
        let inputs = vec![output, p as isize];
        let outputs = computer.run(inputs).unwrap();
        *outputs.last().unwrap() as isize
    }) as usize

    // for p in phase {
    //     let mut computer = Intcode::new(intcodes);
    //     let inputs = vec![output, p as isize];
    //     let outputs = computer.run(inputs).unwrap();
    //     output = *outputs.last().unwrap();
    // }
    // output as usize
}

fn run_amps_recursive(intcodes: &Vec<isize>, phase: Vec<usize>) -> usize {
    //this one needs to PAUSE intcode execution, grab output then pass until
    //halting. Each amp should have phase passed to it, ONCE.
    let mut computers: Vec<Intcode> = (0..5).map(|_| Intcode::new(intcodes)).collect();
    let mut output: Result<isize, isize>  = Ok(phase
        .iter()
        .zip(computers.iter_mut())
        .fold(0isize, |output, (&p, c)| {
            let input = vec![output, p as isize];
           // let output = c.next(input).unwrap();
            output as isize
        }));
    let mut halted = false;
    loop {
        // output = computers.iter_mut().fold(Ok(0isize), |output, c| {
        //     let input = vec![output.unwrap()];
        //     if let Ok(output) = c.next(input) {
        //         Ok(output as isize)
        //     } else {
        //         //halted
        //         halted = true;
        //         output
        //     }
        // });
        // if halted {
        //     break;
        // };
        // println!("{}", output.unwrap());
    }
    output.unwrap() as usize
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
        .map(|p| run_amps(intcodes, p))
        .max()
        .unwrap()
}

pub fn part_2(intcodes: &Vec<isize>) -> usize {
    let amp_count = 5;
    (5..amp_count + 5)
        .permutations(amp_count)
        .map(|p| run_amps_recursive(intcodes, p))
        .max()
        .unwrap()
}
