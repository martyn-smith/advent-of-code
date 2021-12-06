use std::collections::HashMap;

fn grow(i: usize, time: usize) -> usize {
    println!("i = {}, time = {}", i, time);
    if time == 0 {
        1
    } else if time <= 7 {
        2
    } else {
       match i {
           0 => grow(0, time - 7) + grow(2, time - 7),
           _ => grow(0, time - i)
       }
    }
}

pub fn get_input() -> Vec<usize> {
    vec![0]
    // include_str!("../../data/6.txt")
    //         .trim()
    //         .split(',')
    //         .map(|c| c.parse::<usize>().unwrap())
    //         .collect()
}

pub fn part_1(input: &Vec<usize>) -> usize {
    let mut growth = HashMap::new();
    for i in 0..8 {
        growth.insert(i, grow(i, 80));
    }
    dbg!(&growth);
    input.iter()
        .map(|i| growth.get(&i).unwrap())
        .sum()
}
