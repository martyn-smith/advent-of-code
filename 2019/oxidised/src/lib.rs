mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::aoc1;
    use super::aoc2;
    use super::aoc3;
    use super::aoc4;
    use super::aoc5;

    #[test]
    fn day_1() {
        let input = fs::read_to_string("../data/1.txt").unwrap();
        let entries = input.lines()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();
        assert_eq!(3454942, aoc1::get_total_mass(&entries));   
        assert_eq!(5179544, aoc1::get_recursive_total_mass(&entries));
    }

    #[test]
    fn day_2() {
        let input = fs::read_to_string("../data/2.txt").unwrap();
        let mut entries = input.trim().split(',')
                            .map(|l| l.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

        assert_eq!(3409710, aoc2::twelve_oh_two(entries.clone()).unwrap());
        assert_eq!(7912, aoc2::hunt(entries, 19690720).unwrap());
    }

    #[test]
    fn day_3() {
        let input = fs::read_to_string("../data/3.txt").unwrap();
        let entries = aoc3::parse_line(input.lines().nth(0).unwrap().to_string());
        println!("{:?}", entries);
        assert!(false);
    }

    #[test]
    fn day_4() {
        let (low, high) = (271973, 785961);
        assert_eq!(925, aoc4::valid_passwords(low, high));
    }

    #[test]
    fn day_5() {
        let input = fs::read_to_string("../data/5.txt").unwrap();
        let mut entries = input.trim().split(',')
                            .map(|l| l.parse::<isize>().unwrap())
                            .collect::<Vec<isize>>();

        assert_eq!(0, aoc5::run_intcode(entries).unwrap());
    }
}
