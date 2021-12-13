///
/// Advent of Code day 13: Origami
///


use std::collections::HashSet;

pub type paper = HashSet<(usize, usize)>;
pub type foldtype = (Option<usize>, Option<usize>);
pub type foldlist = Vec<(Option<usize>, Option<usize>)>;

fn fold(manual: &mut paper, f: foldtype) {
    let mut to_replace = vec![];
    match f {
        (Some(x), None) => {
            for m in manual.iter() {
                if m.0 > x {
                    let old = (m.0, m.1);
                    let new = (x - (m.0 - x), m.1);
                    to_replace.push((old, new));
                }
            }
        },
        (None, Some(y)) => {
            for m in manual.iter() {
                if m.1 > y {
                    let old = (m.0, m.1);
                    let new = (m.0, y - (m.1 - y));
                    to_replace.push((old, new));
                }
            }
        },
        _ => unreachable!()
    }
    for r in to_replace.drain(0..) {
        manual.remove(&r.0);
        manual.insert(r.1);
    }
}

pub fn get_input() -> (paper, foldlist) {
    let input = include_str!("../../data/13.txt");
    let mut s = input.split("\n\n");
    let mut coords = HashSet::new();
    let mut splits = vec![];
    for l in s.next().unwrap().lines() {
        let mut c = l.split(',');
        let x = c.next().unwrap().parse::<usize>().unwrap();
        let y = c.next().unwrap().parse::<usize>().unwrap();
        coords.insert((x,y));
    }
    for l in s.next().unwrap().lines() {
        let mut c = l.split('=');
        let axis = c.next().unwrap().chars().last().unwrap();
        let pos = c.next().unwrap().parse::<usize>().unwrap();
        let tuple = match axis {
            'x' => (Some(pos), None),
            'y' => (None, Some(pos)),
             _ => panic!("I couldn't correctly parse {}", l)
        };
        splits.push(tuple);
    }
    (coords, splits)
}

pub fn part_1(input: &(paper, foldlist)) -> usize {
    let mut manual = input.0.clone();
    fold(&mut manual, input.1[0]);
    manual.len()
}


pub fn part_2(input: &(paper, foldlist)) {
    let mut manual = input.0.clone();
    for i in input.1.iter() {
        fold(&mut manual, *i);
    }
    let x_max = manual.iter().map(|m| m.0).max().unwrap();
    let y_max = manual.iter().map(|m| m.1).max().unwrap();
    for y in 0..=y_max {
        let mut line = String::new();
        for x in 0..=x_max {
            line.push( if manual.contains(&(x,y)) {'#'} else {'.'});
        }
        println!("{}", line);
    }
}
