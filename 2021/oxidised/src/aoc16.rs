struct Packet {
    v: u8,
    id: u8
}

impl Packet {
    fn from(quads: &[bool]) -> Self {
        let mut q = quads.iter();
        let mut p;
        let mut v = (0..3).rev().fold(0, |(acc, i)| acc + u8::from(q.next().unwrap()) << i);
        let mut id = (0..3).rev().fold(0, |(acc, i)| acc + u8::from(q.next().unwrap()) << i);
        match id {
            4 => {
                let mut r = q.chunks(5);
                while let Some(quint) = q.next() {
                    
                }
            },
            _ => {todo!()}
        }
    }
}

pub fn get_input() -> Vec<bool> {
    let raw = include_str!("../../data/16.txt").trim();
    raw.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).expect("invalid digit"))
        .map(|quad| vec![quad >> 3 != 0,
                         quad >> 2 != 0,
                         quad >> 1 != 0,
                         quad != 0])
        .flatten()
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[bool]) {}
