use super::intcode::Intcode;
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

pub fn get_input() -> Vec<isize> {
    include_str!("../../data/13.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let mut computer = Intcode::from_vec(intcodes).unwrap();
    let outputs = computer.run(vec![]).unwrap();
    let tiles: Vec<Tile> = outputs.chunks(3).map(|t| Tile::new(t).unwrap()).collect();
    //note this does NOT account for overwriting chunks, but that seems to be okay.
    tiles.iter().filter(|&i| TileType::Block == i.tile).count()
}

pub fn part_2(intcodes: &Vec<isize>) -> usize {
    let mut intcodes = intcodes.clone();
    intcodes[0] = 2;
    let mut computer = Intcode::from_vec(&intcodes).unwrap();
    let mut command = vec![0isize];
    let mut paddle = 0isize;
    let mut score = 0;
    while let Some(x) = computer.step(command.clone()) {
        let y = computer.step(vec![]).unwrap();
        let z = computer.step(vec![]).unwrap();
        match (x, y, z) {
            (-1, 0, _) => {
                score = z as usize;
            }
            (_, _, 3) => {
                paddle = x;
            }
            (_, _, 4) => {
                command[0] = (x - paddle).signum();
            }
            _ => {}
        }
    }
    score
}
