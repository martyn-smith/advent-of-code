fn is_valid(i: usize) -> bool {
    let digits = i.to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>();
    let (mut adjacent, mut increasing) = (false, true);
    for d in digits.windows(2) {
        if d[0] > d[1] {
            increasing = false;
            break;
        }
        if d[0] == d[1] {
            adjacent = true;
        }
    }
    adjacent && increasing
}

pub fn valid_passwords(low: usize, high: usize) -> usize {
    (low..high).filter(|&i| is_valid(i)).count()
}