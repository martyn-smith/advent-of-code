mod aoc1;
mod aoc2;
mod aoc4;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::aoc1;
    use super::aoc2;
    use super::aoc4;

    #[test]
    fn day_1() {
        let input = fs::read_to_string("data/1.txt").unwrap();
        let entries = input.lines()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();
        assert_eq!(3454942, aoc1::get_total_mass(&entries));   
        assert_eq!(5179544, aoc1::get_recursive_total_mass(&entries));
    }

    #[test]
    fn day_2() {
        let input = fs::read_to_string("data/2.txt").unwrap();
        let mut entries = input.trim().split(",")
                            .map(|l| l.parse::<usize>().unwrap())
                            .collect();
        aoc2::prepro(&mut entries, 12, 2);
        assert_eq!(3409710, aoc2::run_intcode(&mut entries).unwrap());
        assert_eq!(7912, aoc2::hunt(&mut entries, 19690720).unwrap());
    }

    #[test]
    fn day_4() {
        let (low, high) = (271973, 785961);
        assert_eq!(925, aoc4::valid_passwords(low, high));
    }
}
