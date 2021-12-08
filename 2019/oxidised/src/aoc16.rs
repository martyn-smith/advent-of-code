//use std::fs;

pub fn get_input() -> Vec<usize> {
    let input = include_str!("../../data/16.txt");
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

macro_rules! leading {
    ($src:expr, $n:expr) => {
        {
            let lead_slice = &$src[0..$n];
            lead_slice.iter().fold(0, |acc, x| acc * 10 + x)

        }
    }
}

fn kernel(i: usize, len: usize) -> Vec<i32> {
    // base pattern is 0, 1, 0, -1
    // repeat each value anumber of times equal to position in the output list being considered
    // then discard 0th value
    const BASE: [i32; 4] = [0, 1, 0, -1];
    let reps: Vec<&i32> = BASE.iter()
              .map(|x| std::iter::repeat(x).take(i).collect::<Vec<_>>())
              .flatten()
              .collect();

    let mut k: Vec<i32> = std::iter::repeat(reps.iter())
        .take(len / reps.len() + 1)
        .flatten()
        .cloned()
        .cloned()
        .collect();

    k.drain(1..)
        .collect()
}


fn fft(signal: &mut Vec<usize>) {
    //FIXME: r somehow has length zero
    let r = signal.clone();
    for (i, s) in signal.iter_mut().enumerate() {
        let k = kernel(i, r.len());
        *s = r.iter()
             .zip(k)
             .map(|x| *x.0 as i32 * x.1)
             .sum::<i32>() as usize
    }
}

pub fn part_1(input: &Vec<usize>) -> usize {
    let mut signal = input.clone();
    for _ in 0..100 {
        fft(&mut signal);
    }
    leading!(signal, 8)
}
