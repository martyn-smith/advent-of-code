///
/// Advent of Code day 9: Filesystem tetris
///
use std::fmt;
use std::fmt::{Display, Write};

#[macro_export]
macro_rules! blocks {
    ($blocks:expr) => {{
        use std::fmt::Write;
        $blocks.iter().fold(
            String::with_capacity($blocks.len() * 2),
            |mut output, element| {
                let _ = write!(output, "{}", element);
                output
            },
        )
    }};
}

#[derive(Clone, Debug)]
pub struct Block {
    size: usize,
    free: usize,
    id: usize,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = String::new();
        for _ in 0..self.size {
            let _ = write!(&mut b, "{}", self.id);
        }
        for _ in 0..self.free {
            let _ = write!(&mut b, ".");
        }
        write!(f, "{}", b)
    }
}

impl Block {
    fn new(size: usize, free: usize, id: usize) -> Self {
        Self { size, free, id }
    }
}

fn compact(blocks: &mut Vec<Block>) {
    while let Some(i) = blocks.iter().position(|b| b.free > 0) {
        let mut last = blocks.pop().unwrap();
        let l = blocks.len();
        let free = blocks[i].free;
        if last.size <= free {
            last.free = blocks[i].free - last.size;
            blocks.insert(i + 1, last);
            blocks[i].free = 0;
            blocks[l].free = 0;
        } else {
            let mut split = last.clone();
            split.size = free;
            split.free = 0;
            last.size -= free;
            blocks.insert(i + 1, split);
            blocks[i].free = 0;
            blocks.push(last);
        }
    }
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .fold((0, 0), |acc, b| {
            let a = acc.0;
            let sum = acc.1;
            (
                a + b.size + b.free,
                sum + (b.size * (2 * a + b.size - 1) * b.id / 2),
            )
        })
        .1
}

pub fn get_input() -> Vec<Block> {
    include_str!("../../data/9.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(vec![0usize])
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .map(|(i, ch)| Block::new(ch[0], ch[1], i))
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[Block]) -> usize {
    let mut disk = vec![None; input.iter().map(|b| (b.size + b.free)).sum::<usize>()];
    let mut i = 0usize;
    for b in input {
        for j in i..i + b.size {
            disk[j] = Some(b.id);
        }
        i += b.size + b.free;
    }
    let mut blocks = input.to_owned();
    compact(&mut blocks);
    //println!(
    //    "{}",
    //    blocks.iter().fold(String::new(), |mut out, e| {
    //        let _ = write!(&mut out, "{}", e);
    //        out
    //    })
    //);
    checksum(&blocks)
}

// Done. This is such a painful approach, however.
// I don't like using a mixture of .iter_mut() and [i] access but this may be the alternative,
// and it's horrible in order to avoid bad accesses from length-changes.
// note as a general rule, if answer is too high - file moves have been missed.
pub fn part_2(input: &[Block]) -> usize {
    let mut blocks = input.to_owned();
    // likely unnecessary, but here for safety. Not it's in increasing order so we can pop.
    blocks.sort_by_key(|b| b.id);
    for i in (0..blocks.len()).rev() {
        'a: loop {
            let mut terminate = true;
            'b: for j in 0..i {
                if blocks[j].free >= blocks[i].size {
                    let f = blocks.remove(i);
                    terminate = false;
                    //println!("file {} moved from {} to {}", f.id, i, j);
                    blocks[i - 1].free += f.size + f.free;
                    blocks.insert(j + 1, f);
                    blocks[j + 1].free = blocks[j].free - blocks[j + 1].size;
                    blocks[j].free = 0;
                    println!("{}", blocks!(blocks));
                    break 'b;
                }
            }
            if terminate {
                break 'a;
            }
        }
    }
    checksum(&blocks)
}
