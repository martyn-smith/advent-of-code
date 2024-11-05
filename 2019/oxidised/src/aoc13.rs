use crate::intcode;
use crate::intcode::{Computer, From, Program};
//use std::fs;

#[derive(PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

struct Tile {
    x: usize,
    y: usize,
    tile: TileType,
}

impl Tile {
    fn new(t: &[isize]) -> Result<Self, String> {
        let t = (t[0], t[1], t[2]);
        Ok(Tile {
            x: t.0 as usize,
            y: t.1 as usize,
            tile: match t.2 {
                0 => TileType::Empty,
                1 => TileType::Wall,
                2 => TileType::Block,
                3 => TileType::Paddle,
                4 => TileType::Ball,
                _ => return Err(format!("invalid tile type! {:?}", t)),
            },
        })
    }
}

fn next(command: isize, computer: &mut Computer, program: &mut Program) -> Option<[isize; 3]> {
    let mut command = vec![command];
    let x = computer.next(program, Some(&mut command)).unwrap()?;
    let y = computer.next(program, None).unwrap()?;
    let z = computer.next(program, None).unwrap()?;
    Some([x, y, z])
}

pub fn get_input() -> Vec<isize> {
    include_str!("../../data/13.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &[isize]) -> usize {
    let outputs = intcode::solve(intcodes.into(), None).unwrap();
    let tiles: Vec<Tile> = outputs.chunks(3).map(|t| Tile::new(t).unwrap()).collect();
    //note this does NOT account for overwriting chunks, but that seems to be okay.
    tiles.iter().filter(|&i| TileType::Block == i.tile).count()
}

pub fn part_2(intcodes: &[isize]) -> usize {
    let mut intcodes = intcodes.to_owned();
    intcodes[0] = 2;
    let mut program = Program::from(&intcodes[..]);
    let mut computer = Computer::new();
    let mut command = 0;
    let mut paddle = 0isize;
    let mut score = 0;
    while let Some([x, y, z]) = next(command, &mut computer, &mut program) {
        match (x, y, z) {
            (-1, 0, _) => {
                score = z as usize;
            }
            (_, _, 3) => {
                paddle = x;
            }
            (_, _, 4) => {
                command = (x - paddle).signum();
            }
            _ => {}
        }
    }
    score
}
