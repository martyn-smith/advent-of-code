use itertools::Itertools;

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
