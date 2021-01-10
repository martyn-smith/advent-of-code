use itertools::Itertools;

fn add(intcodes: &mut Vec<usize>, pos: usize) {
    let (a_pos, b_pos, write_pos) = (intcodes[pos+1], intcodes[pos+2], intcodes[pos+3]);
    let a = intcodes[a_pos];
    let b = intcodes[b_pos];
    intcodes[write_pos] = a + b;
}

fn mult(intcodes: &mut Vec<usize>, pos: usize) {
    let (a_pos, b_pos, write_pos) = (intcodes[pos+1], intcodes[pos+2], intcodes[pos+3]);
    let a = intcodes[a_pos];
    let b = intcodes[b_pos];
    intcodes[write_pos] = a * b;
}

fn prepro(intcodes: &mut Vec<usize>, noun: usize, verb: usize) {
    intcodes[1] = noun;
    intcodes[2] = verb;
}

pub fn run_intcode(mut intcodes: Vec<usize>) -> Result<usize, usize> {
    let mut i = 0usize;
    'a: loop {
       // println!("{}", intcodes[i]);
        match intcodes[i] {
            1 => add(&mut intcodes, i),
            2 => mult(&mut intcodes, i),
            99 => {break 'a;},
            _ => {return Err(0);}
        }
        i += 4;
    }
    Ok(intcodes[0])
}

pub fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("../data/data/2.txt").unwrap();
    input.trim().split(',')
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
}

pub fn part_1(mut program: Vec<usize>) -> Result<usize, usize> {
    prepro(&mut program, 12, 2);
    run_intcode(program)
}

pub fn part_2(program: Vec<usize>, target: usize) -> Option<usize> {
    let range = (79..80).cartesian_product(12..13);
    for it in range {
        let mut candidate = program.clone();
        prepro(&mut candidate, 79, 12);
        if let Ok(r) = run_intcode(candidate) {
            if r == target {
                return Some(it.0 * 100 + it.1);
            }
        }
    }
    None
}