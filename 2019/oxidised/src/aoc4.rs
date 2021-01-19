fn is_valid(num: usize) -> bool {
    //todo: very slow, probably due to this initial collection. Consider ways to avoid.
    let digits = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let (mut adjacent, mut decreasing) = (false, false);
    'a: for d in digits.windows(2) {
        match (d[1] - d[0]).signum() {
            1 => {}
            -1 => {
                decreasing = true;
                break 'a;
            }
            0 => {
                adjacent = true;
            }
            _ => {
                panic!();
            }
        }
    }
    adjacent && !decreasing
}

fn enhanced_is_valid(i: usize) -> bool {
    let digits = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let (mut adjacent, mut decreasing) = (false, false);
    let mut r = vec![0i32];
    'a: for d in digits {
        // implement a state machine
        match (d - r.last().unwrap()).signum() {
            1 => {
                if r.len() == 2 {
                    adjacent = true;
                }
                r = vec![d];
            }
            -1 => {
                decreasing = true;
                break 'a;
            }
            0 => {
                r.push(d);
            }
            _ => {
                panic!();
            }
        }
    }
    if r.len() == 2 {
        adjacent = true;
    }
    adjacent && !decreasing
}

pub fn get_input() -> (usize, usize) {
    (271973, 785961)
}

pub fn part_1(input: &(usize, usize)) -> usize {
    let low = input.0;
    let high = input.1;
    (low..high).filter(|&i| is_valid(i)).count()
}

pub fn part_2(input: &(usize, usize)) -> usize {
    let low = input.0;
    let high = input.1;
    (low..high).filter(|&i| enhanced_is_valid(i)).count()
}
