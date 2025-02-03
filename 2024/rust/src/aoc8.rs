use std::collections::HashSet;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Antenna {
    pos: [usize; 2],
    freq: char,
}

impl Antenna {
    fn new(pos: [usize; 2], freq: char) -> Self {
        Self { pos, freq }
    }
}

fn get_pairs(antennae: &[Antenna]) -> Vec<[&Antenna; 2]> {
    antennae
        .iter()
        .flat_map(|a| {
            antennae
                .iter()
                .filter_map(|b| {
                    if b.freq == a.freq && a.pos != b.pos {
                        Some([a, b])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_vec(a: &Antenna, b: &Antenna) -> [i32; 2] {
    [
        b.pos[0] as i32 - a.pos[0] as i32,
        b.pos[1] as i32 - a.pos[1] as i32,
    ]
}

fn get_line(r: &Antenna, ab: &[i32; 2], limits: &[usize; 2]) -> Vec<[i32; 2]> {
    let (width, depth) = (limits[0], limits[1]);
    let mut p = [r.pos[0] as i32, r.pos[1] as i32];
    let mut antinodes = vec![];
    //search forward
    while p[0] >= 0 && p[0] < width as i32 && p[1] >= 0 && p[1] < depth as i32 {
        antinodes.push(p);
        p[0] += ab[0];
        p[1] += ab[1];
    }
    p = [r.pos[0] as i32, r.pos[1] as i32];
    while p[0] >= 0 && p[0] < width as i32 && p[1] >= 0 && p[1] < depth as i32 {
        antinodes.push(p);
        p[0] -= ab[0];
        p[1] -= ab[1];
    }
    antinodes
}

pub fn get_input() -> ([usize; 2], Vec<Antenna>) {
    let input = include_str!("../../data/8.txt");
    let width = input.lines().next().unwrap().len();
    let depth = input.lines().count();
    let antennae = input
        .lines()
        .enumerate()
        .flat_map(|(i, r)| {
            r.chars().enumerate().flat_map(move |(j, c)| {
                if c != '.' {
                    //if c == 'A' {
                    Some(Antenna::new([j, i], c))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    ([width, depth], antennae)
}

pub fn part_1(input: &([usize; 2], Vec<Antenna>)) -> usize {
    let (width, depth) = (input.0[0], input.0[1]);
    let antennae = &input.1;
    let pairs = get_pairs(antennae);
    let antinodes = pairs
        .iter()
        .flat_map(|p| {
            let a = &p[0];
            let b = &p[1];
            let ab = get_vec(a, b);
            [
                [b.pos[0] as i32 + ab[0], b.pos[1] as i32 + ab[1]],
                [a.pos[0] as i32 - ab[0], a.pos[1] as i32 - ab[1]],
            ]
        })
        .filter(|an| an[0] >= 0 && an[0] < width as i32 && an[1] >= 0 && an[1] < depth as i32)
        .collect::<HashSet<_>>();

    //println!(
    //    "{}",
    //    (0..depth).fold(String::new(), |mut board, r| {
    //        let row = (0..width).fold(String::new(), |mut rw, c| {
    //            let e = if antinodes.contains(&[c as i32, r as i32]) {
    //                '#'
    //            } else if let Some(a) = antennae.iter().find(|a| a.pos == [c, r]) {
    //                a.freq
    //            } else {
    //                '.'
    //            };
    //            let _ = write!(&mut rw, "{}", e);
    //            rw
    //        });
    //        let _ = writeln!(&mut board, "{}", row);
    //        board
    //    })
    //);
    antinodes.len()
}

pub fn part_2(input: &([usize; 2], Vec<Antenna>)) -> usize {
    let (width, depth) = (input.0[0], input.0[1]);
    let antennae = &input.1;
    let pairs = get_pairs(antennae);
    let antinodes = pairs
        .iter()
        .flat_map(|p| {
            let a = &p[0];
            let b = &p[1];
            // I was expecting to have to get a LCD for this to get all possible points.
            // Oddly, it doesn't seem necessary.
            let ab = get_vec(a, b);
            vec![
                get_line(a, &ab, &[width, depth]),
                get_line(b, &ab, &[width, depth]),
            ]
            .into_iter()
            .flatten()
        })
        .collect::<HashSet<_>>();
    //println!(
    //    "{}",
    //    (0..depth).fold(String::new(), |mut board, r| {
    //        let row = (0..width).fold(String::new(), |mut rw, c| {
    //            let e = if antinodes.contains(&[c as i32, r as i32]) {
    //                '#'
    //            } else if let Some(a) = antennae.iter().find(|a| a.pos == [c, r]) {
    //                a.freq
    //            } else {
    //                '.'
    //            };
    //            let _ = write!(&mut rw, "{}", e);
    //            rw
    //        });
    //        let _ = writeln!(&mut board, "{}", row);
    //        board
    //    })
    //);
    antinodes.len()
}
