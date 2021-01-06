fn is_valid(num: usize) -> bool {
    //todo: very slow, probably due to this initial collection. Consider ways to avoid.
    let digits = num.to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap() as i32)
                        .collect::<Vec<i32>>();
    let (mut adjacent, mut decreasing) = (false, false);
    'a: for d in digits.windows(2) {
        match (d[0] - d[1]).signum() {
            1 => {decreasing = true; break 'a;},
            -1 => {},
            0 => {adjacent = true;},
            _ => {panic!();}
        }
    }
    adjacent && !decreasing
}

enum Reader {
    new,
}

fn enhanced_is_valid(i: usize) -> bool {
    let digits = i.to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>();
    let (mut adjacent, mut decreasing, mut larger_group) = (false, false, false);
    let mut r = Reader::new;
    for d in digits {
        // implement a state machine
        // match r {
        //     // new => r = Reader::nondecreasing();
        //     // nondecreasing => 
        // }
    }
    adjacent && !decreasing && !larger_group
}

pub fn valid_passwords(low: usize, high: usize) -> usize {
    (low..high).filter(|&i| is_valid(i)).count()
}

pub fn enhanced_valid_passwords(low: usize, high: usize) -> usize {
    (low..high).filter(|&i| enhanced_is_valid(i)).count()
}