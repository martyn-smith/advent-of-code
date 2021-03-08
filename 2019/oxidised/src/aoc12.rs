use itertools::Itertools;
// use num::integer::lcm;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Moon {
    x: isize,
    y: isize,
    z: isize,
    v_x: isize,
    v_y: isize,
    v_z: isize,
    h_x: HashSet<(isize, isize)>,
    h_y: HashSet<(isize, isize)>,
    h_z: HashSet<(isize, isize)>,
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
            h_x: HashSet::new(),
            h_y: HashSet::new(),
            h_z: HashSet::new(),
        }
    }

    fn e(&self) -> usize {
        ((self.x.abs() + self.y.abs() + self.z.abs())
            * (self.v_x.abs() + self.v_y.abs() + self.v_z.abs())) as usize
    }

    fn gravity(&mut self, moon: &Moon) {
        self.v_x += (moon.x - self.x).signum();
        self.v_y += (moon.y - self.y).signum();
        self.v_z += (moon.z - self.z).signum();
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
    // vec![
    //     Moon::new(-1, 0, 2),
    //     Moon::new(2, -10, -7),
    //     Moon::new(4, -8, 8),
    //     Moon::new(3, 5, -1),
    // ]
}

pub fn part_1(input: &Vec<Moon>) -> usize {
    let mut moons: Vec<Moon> = input.clone();
    for _ in 0..1000 {
        // apply gravity
        // TODO: get a (&mut Moon, &Moon) from permutations
        for i in (0..moons.len()).permutations(2) {
            let m = moons[i[1]].clone();
            moons[i[0]].gravity(&m);
        }
        // apply velocity
        for m in &mut moons {
            m.velocity();
        }
    }
    moons.iter().map(|m| m.e()).sum()
}

// pub fn part_2(input: &Vec<Moon>) -> usize {
//     let mut moons: Vec<Moon> = input.clone();
//     let mut ctr = 0;
//     let mut period: [Option<usize>; 3] = [None; 3];
//     loop {
//         for i in (0..moons.len()).permutations(2) {
//             let m = moons[i[1]].clone();
//             moons[i[0]].gravity(&m);
//         }
//         for m in &mut moons {
//             m.velocity();
//         }
//         if moons.iter_mut().all(|m| !m.h_x.insert((m.x, m.v_x))) {
//             println!("found x at {}", ctr);
//             period[0] = Some(ctr);
//         }
//         if moons.iter_mut().all(|m| !m.h_y.insert((m.y, m.v_y))) {
//             println!("found y at {}", ctr);
//             period[1] = Some(ctr);
//         }
//         if moons.iter_mut().all(|m| !m.h_z.insert((m.z, m.v_z))) {
//             println!("found z at {}", ctr);
//             period[2] = Some(ctr);
//         }
//         if let [Some(_), Some(_), Some(_)] = period {
//             break;
//         }
//         if ctr % 100_0 == 0 {
//             println!("{}", ctr);
//         }
//         ctr += 1;
//     }
//     println!("{:?}", period);
//     period.iter().fold(1, |acc, x| lcm(acc, x.unwrap()))
// }
