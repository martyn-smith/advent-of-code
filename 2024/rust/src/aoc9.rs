#[derive(Clone, Debug)]
pub struct Block {
    size: usize,
    free: usize,
    id: usize,
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
        //dbg!(&blocks[i], &last);
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
        for j in i..i + b.size as usize {
            disk[j] = Some(b.id);
        }
        i += b.size + b.free;
    }
    let mut blocks = input.to_owned();
    compact(&mut blocks);
    blocks
        .into_iter()
        .fold((0, 0), |acc, b| {
            let a = acc.0;
            let sum = acc.1;
            (a + b.size, sum + (b.size * (2 * a + b.size - 1) * b.id / 2))
        })
        .1
}
