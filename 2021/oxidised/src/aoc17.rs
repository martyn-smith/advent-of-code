const TRIANGLE_NUMS: [usize; 18] = [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153];

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

struct Probe {
    pos: Point,
    v: Point
}

impl Probe {
    fn new(v: Point) -> Self {
        Probe {
            pos: Point {0, 0},
            v: v
        }
    }

    fn step(&mut self) {
        self.pos.x += self.v.x;
        self.pos.y += self.v.y;
        self.drag();
        self.gravity();
    }

    fn drag(&mut self) {
        self.v.x -= self.v.x.signum();
    }

    fn gravity(&mut self) {
        self.v.y -= 1;
    }
}

pub fn get_input() ->  (Point, Point) {
    //returns the UL and LR corners of the target area
    (Point {153, -75}, Point {199, -114})
}

pub fn part_1(target_rng: &(Point, Point)) -> isize {
    //create a probe that, when fired, passes through the rectangle defined by target
    // x velocity must be at least as much as the number whose triangle n(n+1)/2 sums to x_min,
    // at most as much as x_max
    let x_min = TRIANGLE_NUMS.iter().find(|&i| i >= target_rng.0.x).unwrap() as isize;
    let x_max = target_rng.1.x;
    //starting y velocity must be such that, after at most the number of timesteps that results
    //in x being zero, downward velocty must be less than the height of the box.
    let v_y_min = target_rng.1.y - target.rng.0.y;
    let contains : |target_rng, pos| = pos.x >= target_rng.0.x && pos.x <= target_rng.1.x
                                     && pos.y <= target_rng.0.y && pos.y >= target_rng.1.y;
    //let's just start with random guesses for y
    for v_y in 0..100 {
}
