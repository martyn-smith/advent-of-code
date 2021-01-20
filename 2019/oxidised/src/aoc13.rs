use super::intcode::Intcode;
use std::fs;

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
    let input = fs::read_to_string("../data/13.txt").unwrap();
    input
        .trim()
        .split(',')
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part_1(intcodes: &Vec<isize>) -> usize {
    let mut computer = Intcode::new(intcodes);
    let outputs = computer.run(vec![]).unwrap();
    let tiles: Vec<Tile> = outputs.chunks(3).map(|t| Tile::new(t).unwrap()).collect();
    //note this does NOT account for overwriting chunks, but that seems to be okay.
    tiles.iter().filter(|&i| TileType::Block == i.tile).count()
}
