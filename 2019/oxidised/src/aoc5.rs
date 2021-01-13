use std::fs;

fn get_value(intcodes: &Vec<isize>, pos: usize, mode_arg: isize) -> isize {
    match mode_arg {
        //position mode
        0 => {
            let ptr = intcodes[pos] as usize;
            intcodes[ptr]
        }
        1 => intcodes[pos],
        _ => {
            panic!();
        }
    }
}

fn add(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let write_pos = intcodes[pos + 3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    intcodes[write_pos] = a + b;
    4
}

fn mult(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    let write_pos = intcodes[pos + 3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    intcodes[write_pos] = a * b;
    4
}

fn input(intcodes: &mut Vec<isize>, pos: usize, input: &str) -> usize {
    let write_pos = intcodes[pos + 1] as usize;
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    let write_val = input.trim().parse::<isize>().unwrap();
    intcodes[write_pos] = write_val;
    2
}

fn output(intcodes: &mut Vec<isize>, pos: usize, outputs: &mut Vec<isize>) -> usize {
    let mode_arg = intcodes[pos] / 100;
    let read_pos = get_value(&intcodes, pos + 1, mode_arg);
    outputs.push(read_pos);
    2
}

fn jt(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    //Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    if a != 0 {
        b as usize - pos
    } else {
        3
    }
}

fn jf(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    if a == 0 {
        b as usize - pos
    } else {
        3
    }
}

fn lt(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    // Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let write_pos = intcodes[pos + 3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    intcodes[write_pos] = if a < b { 1 } else { 0 };
    4
}

fn eq(intcodes: &mut Vec<isize>, pos: usize) -> usize {
    // Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    let write_pos = intcodes[pos + 3] as usize;
    let mode_args = intcodes[pos] / 100;
    let a = get_value(&intcodes, pos + 1, mode_args % 10);
    let b = get_value(&intcodes, pos + 2, (mode_args / 10) % 10);
    intcodes[write_pos] = if a == b { 1 } else { 0 };
    4
}

fn run_intcode(mut intcodes: Vec<isize>, system_id: &str) -> Result<Vec<isize>, isize> {
    let mut i = 0usize;
    let mut outputs: Vec<isize> = vec![];
    'a: loop {
        let adv = match intcodes[i] % 100 {
            1 => add(&mut intcodes, i),
            2 => mult(&mut intcodes, i),
            3 => input(&mut intcodes, i, system_id),
            4 => output(&mut intcodes, i, &mut outputs),
            5 => jt(&mut intcodes, i),
            6 => jf(&mut intcodes, i),
            7 => lt(&mut intcodes, i),
            8 => eq(&mut intcodes, i),
            99 => {
                break 'a;
            }
            _ => {
                return Err(-1);
            }
        };
        i += adv;
    }
    Ok(outputs)
}

pub fn get_input() -> Vec<isize> {
    let input = fs::read_to_string("../data/5.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &Vec<isize>) -> Result<isize, isize> {
    let mut intcodes = intcodes.clone();
    let outputs = run_intcode(intcodes, "1").unwrap();
    assert!(outputs.iter().filter(|&&i| i != 0).count() <= 1, 
            "ERROR: intcode returned too many non-zero status codes");
    Ok(*outputs.iter().last().unwrap())
}

pub fn part_2(intcodes: &Vec<isize>) -> Result<isize, isize> {
    let mut intcodes = intcodes.clone();
    let outputs = run_intcode(intcodes, "5").unwrap();
    assert!(outputs.iter().filter(|&&i| i != 0).count() <= 1, 
           "ERROR: intcode returned too many non-zero status codes");
    Ok(*outputs.iter().last().unwrap())
}
