///
/// Advent of Code day 14: exponential polymeriseration
///

use std::iter::FromIterator;
use std::collections::HashMap;
use std::ops::Index;


const LETTERS: &'static str = "ABCDEFGHIJKLNOPQRSTUVWXYZ";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pair {
    ch: [char; 2]
}

impl Pair {
    fn new(s: &str) -> Self {
        let mut w = s.chars();
        Pair {
            ch: [w.next().unwrap(), w.next().unwrap()]
        }
    }

    fn contains(&self, c: char) -> bool {
        self.ch[0] == c || self.ch[1] == c
    }

    fn both(&self, c: char) -> bool {
        self.ch[0] == c && self.ch[1] == c
    }
}

impl Index<usize> for Pair {
    type Output = char;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.ch[0],
            1 => &self.ch[1],
            _ => panic!("invalid index {} for pair", i)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Counts {
    cts: HashMap<Pair, usize>
}

impl Counts {
    fn new(l: &str) -> Self {
        let mut cts = HashMap::new();
        for i in 0..=l.len() - 2 {
            let p = Pair::new(&l[i..i+2]);
            if let Some(n) = cts.get_mut(&p) {
                *n += 1;
            } else {
                cts.insert(p, 1);
            }
        }
        Counts{cts}
    }
}

struct Reaction {
    pin: Pair,
    pout: (Pair, Pair)
}

impl Reaction {
    fn new(l: &str) -> Self {
        let mut s = l.split(" -> ");
        let pin = Pair::new(s.next().unwrap());
        let cout = s.next().unwrap();
        let pout = (Pair::new(&format!("{}{}", pin[0], cout)[..]),
                    Pair::new(&format!("{}{}", cout, pin[1])[..]));
        Reaction {
            pin,
            pout
        }
    }
}

fn get_char_count(c: char, counts: &Counts) -> usize {
    counts.cts.iter()
        .map(|(pair, count)|
             if pair.both(c) {
                    *count
             } else if pair.contains(c) {
                    *count / 2
             } else {
                 0
             })
        .sum()
}

fn polymerise(mut counts: Counts, reactions: &HashMap<Pair, (Pair, Pair)>) -> Counts {
    let mut next = HashMap::new();
    for (pair, count) in counts.cts.drain() {
        let pairs = reactions.get(&pair).unwrap();
        for p in [pairs.0, pairs.1].iter() {
            if let Some(n) = next.get_mut(p) {
                *n += count;
            } else {
                next.insert(p.clone(), count);
            }
        }
    }
    Counts{cts: next}
}

pub fn get_input() -> (Counts, HashMap<Pair, (Pair, Pair)>) {
    let mut s = include_str!("../../data/14.txt").split("\n\n");
    let template = Counts::new(s.next().unwrap());
    let reactions = HashMap::from_iter(
                            s.next().unwrap().lines().map(|l| {
                                let r = Reaction::new(l);
                                (r.pin, r.pout)
                            })
                        );
    (template, reactions)
}

/*
 * You somehow need to deal with first and last chars. Or you could... not.
 */

pub fn part_1(input: &(Counts, HashMap<Pair, (Pair, Pair)>)) -> usize {
    let count_0 = input.0.clone();
    let reactions = &input.1;
    let count_n = (0..10usize)
                    .fold(count_0, |counts, _| {
                        let count = polymerise(counts, reactions);
//                        println!("after step {}: {}", i + 1, count.cts.values().sum::<usize>() + 1);
                        count
                    });

    let counts = LETTERS.chars()
                        .map(|c| get_char_count(c, &count_n))
                        .filter(|&n| n > 0)
                        .collect::<Vec<usize>>();
    counts.iter().max().unwrap() - counts.iter().min().unwrap() + 1
}

pub fn part_2(input: &(Counts, HashMap<Pair, (Pair, Pair)>)) -> usize {
    let count_0 = input.0.clone();
    let reactions = &input.1;
    let count_n = (0..40usize)
                    .fold(count_0, |counts, _| polymerise(counts, reactions));

    let counts = LETTERS.chars()
                        .map(|c| get_char_count(c, &count_n))
                        .filter(|&n| n > 0)
                        .collect::<Vec<usize>>();

    counts.iter().max().unwrap() - counts.iter().min().unwrap() - 1
}

