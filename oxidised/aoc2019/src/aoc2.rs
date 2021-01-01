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

pub fn prepro(intcodes: &mut Vec<usize>, noun: usize, verb: usize) {
    intcodes[1] = noun;
    intcodes[2] = verb;
}

pub fn run_intcode(mut intcodes: &mut Vec<usize>) -> Result<usize, &'static str> {
    let mut i = 0usize;
    'a: loop {
        match intcodes[i] {
            1 => add(&mut intcodes, i),
            2 => mult(&mut intcodes, i),
            99 => {break 'a;},
            _ => {return Err("intcode panicked!");}
        }
        i += 4;
    }
    Ok(intcodes[0])
}

pub fn hunt(mut intcodes: &mut Vec<usize>, target: usize) -> Option<usize> {
    let range = (0..100).cartesian_product(0..100);
    for it in range {
        //println!("trying {} {}", it.0, it.1);
        prepro(&mut intcodes, it.0, it.1);
        let q = run_intcode(&mut intcodes);
        //println!("{:?}", q);
        if let Ok(r) = q {//run_intcode(&mut intcodes) {
            println!("{}", r);
            if r == target {
                return Some(it.0 * 100 + it.1);
            }
        }
    }
    None
}