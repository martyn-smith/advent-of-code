use std::collections::HashSet;

pub fn get_input() -> Vec<(i32, i32)> {
    include_str!("../../data/3.txt")
            .chars()
            .map(|c|
                match c {
                '>' => (1,0),
                '<' => (-1,0),
                '^' => (0,1),
                'v' => (0,-1),
                _ => unreachable!("{}", c)
                })
        .collect()
}

pub fn part_1(moves: &Vec<(i32, i32)>) -> usize {
    let mut positions = HashSet::new();
    let mut pos = (0,0);
    for m in moves.iter() {
        pos.0 += m.0;
        pos.1 += m.1;
        positions.insert(pos);
    }
    positions.len()
}

pub fn part_2(moves: &Vec<(i32, i32)>) -> usize {
    let mut positions = HashSet::new();
    let (mut h_pos, mut r_pos) = ((0,0), (0,0));
    for m in moves.chunks(2) {
        h_pos.0 += m[0].0;
        h_pos.1 += m[0].1;
        r_pos.0 += m[1].0;
        r_pos.1 += m[1].1;
        positions.insert(h_pos);
        positions.insert(r_pos);
    }
    positions.len()
}
