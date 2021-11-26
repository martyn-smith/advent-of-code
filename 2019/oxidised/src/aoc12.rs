use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Moon {
    x: isize,
    y: isize,
    z: isize,
    v_x: isize,
    v_y: isize,
    v_z: isize,
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon {
            x: x,
            y: y,
            z: z,
            v_x: 0,
            v_y: 0,
            v_z: 0,
        }
    }

    fn e(&self) -> usize {
        ((self.x.abs() + self.y.abs() + self.z.abs())
            * (self.v_x.abs() + self.v_y.abs() + self.v_z.abs())) as usize
    }

    fn velocity(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
        self.z += self.v_z;
    }
}

pub fn get_input() -> Vec<Moon> {
    vec![
        Moon::new(17, -7, -11),
        Moon::new(1, 4, -1),
        Moon::new(6, -2, -6),
        Moon::new(19, 11, 9),
    ]
}

fn gravity(moons: &mut Vec<Moon>, i: usize, j: usize) {
    let a_x = (moons[i].x - moons[j].x).signum();
    let a_y = (moons[i].y - moons[j].y).signum();
    let a_z = (moons[i].z - moons[j].z).signum();

    moons[i].v_x -= a_x;
    moons[i].v_y -= a_y;
    moons[i].v_z -= a_z;

    moons[j].v_x += a_x;
    moons[j].v_y += a_y;
    moons[j].v_z += a_z;
}

pub fn part_1(input: &Vec<Moon>) -> usize {
    let mut moons: Vec<Moon> = input.clone();
    for _ in 0..1000 {
        // apply gravity
        for p in (0..moons.len()).combinations(2) {
            let (i, j) = (p[0], p[1]);
            gravity(&mut moons, i, j);
        }
        // apply velocity
        for m in &mut moons {
            m.velocity();
        }
    }
    moons.iter().map(|m| m.e()).sum()
}

pub fn part_2(input: &Vec<Moon>) -> usize {
    let mut moons: Vec<Moon> = input.clone();
    let mut ctr = 0;
    let mut period: [Option<usize>; 3] = [None; 3];
    let mut x_history = HashSet::new();
    let mut y_history = HashSet::new();
    let mut z_history = HashSet::new();
    loop {
        // apply gravity
        for p in (0..moons.len()).combinations(2) {
            let (i, j) = (p[0], p[1]);
            gravity(&mut moons, i, j);
        }
        // apply velocity
        for m in &mut moons {
            m.velocity();
        }
        let x = moons.iter().map(|m| (m.x, m.v_x)).collect::<Vec<_>>();
        let y = moons.iter().map(|m| (m.y, m.v_y)).collect::<Vec<_>>();
        let z = moons.iter().map(|m| (m.z, m.v_z)).collect::<Vec<_>>();
        if let None = period[0] {
            if !x_history.insert(x) {
                period[0] = Some(ctr);
            }
        }
        if let None = period[1] {
            if !y_history.insert(y) {
                period[1] = Some(ctr);
            }
        }
        if let None = period[2] {
            if !z_history.insert(z) {
                period[2] = Some(ctr);
            }
        }
        if let [Some(_), Some(_), Some(_)] = period {
            break;
        }
        if ctr % 100_0 == 0 {
            //println!("{}", ctr);
        }
        ctr += 1;
    }
    period.iter().fold(1, |acc, x| lcm(acc, x.unwrap()))
}
