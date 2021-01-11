enum Direction {
    Horizontal,
    Vertical, 
    Terminal
}

struct Segment {
    direction: Direction,
    pos: (i32, i32)
}

pub fn parse_line(line: String) -> Vec<(i32, i32)> {
    let mut wire = vec![(0,0)];

    for l in line.trim().split(',') {
        let mag = str::parse::<i32>(&l[1..]).unwrap();
        let vec = match l.chars().nth(0).unwrap() {
            'L' => (-mag, 0i32),
            'R' => (mag, 0i32),
            'U' => (0i32, mag),
            'D' => (0i32, -mag),
            _ => {panic!()}
        };
        let last = wire[wire.len()-1];
        let pos = (last.0 + vec.0, last.1 + vec.1);
        wire.push(pos);
    }
    wire
}

fn intersection(s1: Windows<Segment>, s2: Windows<Segment>) -> bool {
    let is_horiztonal = |l| {l[0].0 == l[1].0};
    let is_vertical = |l| {l[0].1 == l[1].1};
    false
}