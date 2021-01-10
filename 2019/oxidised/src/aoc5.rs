use std::io;

fn get_value(intcodes: &Vec<isize>, pos: usize, mode_arg: isize) -> isize {
    match mode_arg { //position mode
        0 => {
            let ptr = intcodes[pos] as usize;
            intcodes[ptr]
        },
        1 => {intcodes[pos]},
        _ => {panic!();}
    }
}

fn add(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let write_pos = intcodes[pos+3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos+1, mode_args % 10);
    let b = get_value(&intcodes, pos+2, (mode_args / 10) % 10);
    intcodes[write_pos] = a + b;
    4
}

fn mult(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let write_pos = intcodes[pos+3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos+1, mode_args % 10);
    let b = get_value(&intcodes, pos+2, (mode_args / 10) % 10);
    intcodes[write_pos] = a * b;
    4
}

fn input(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let write_pos = intcodes[pos+1] as usize;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let write_val = input.trim().parse::<isize>().unwrap();
    intcodes[write_pos] = write_val;
    2
}

fn output(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let read_pos = intcodes[pos+1] as usize;
    println!("{}", intcodes[read_pos]);
    2
}

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/data/5.txt").unwrap();
    input.trim().split(',')
            .map(|l| l.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
}

pub fn run_intcode(mut intcodes: Vec<isize>) -> Result<isize, isize> {
    let mut i = 0usize;
    'a: loop {
        println!("{}", intcodes[i]);
        let adv = match intcodes[i] % 100 {
             1 => add(&mut intcodes, i),
             2 => mult(&mut intcodes, i),
             3 => input(&mut intcodes, i),
             4 => output(&mut intcodes, i),
             99 => {break 'a;},
             _ => {return Err(-1);}
         };
         i += adv;
     }
    Ok(intcodes[0])
}