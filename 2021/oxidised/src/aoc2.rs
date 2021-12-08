///
/// Advent of Code day 2: Piloting a submarine
///

fn get_vector(l: &str) -> (i32, i32) {
    let mut l = l.split(" ");
    let sign = match l.next().unwrap() {
        "forward" => (1, 0),
        "backward" => (-1, 0),
        "up" => (0, -1),
        "down" => (0, 1),
        _ => unreachable!()
    };
    let mag = l.next().unwrap().parse::<i32>().unwrap();
    (sign.0 * mag, sign.1 * mag)
}

fn aim(pos: (i32, i32, i32), v: (i32, i32)) -> (i32, i32, i32) {
    match (v.0 != 0, v.1 != 0) {
        //adjust aim
        (false, true) => (pos.0, pos.1, pos.2 + v.1),
        //adjust pos
        (true, false) => (pos.0 + v.0, pos.1 + (pos.2 * v.0), pos.2),
        _ => unreachable!()
    }
}

pub fn get_input() -> Vec<(i32, i32)> {
    include_str!("../../data/2.txt")
        .lines()
        .map(|l| get_vector(l))
        .collect()
}

pub fn part_1(input: &Vec<(i32, i32)>) -> usize {
    let pos = input.iter()
                    .fold((0i32, 0i32), |pos, v| (pos.0 + v.0, pos.1 + v.1));
    (pos.0 * pos.1) as usize
}

pub fn part_2(input: &Vec<(i32, i32)>) -> usize {
    let pos = input.iter()
                    .fold((0i32, 0i32, 0i32), |pos, &v| aim(pos, v));
    (pos.0 * pos.1) as usize
}
