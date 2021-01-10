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
        let input = aoc1::get_input();
        assert_eq!(3454942, aoc1::part_1(&input));   
        assert_eq!(5179544, aoc1::part_2(&input));
    }

    #[test]
    fn day_2() {
        let input = aoc2::get_input();
        assert_eq!(3409710, aoc2::part_1(input.clone()).unwrap());
        assert_eq!(7912, aoc2::part_2(input, 19690720).unwrap());
    }

    #[test]
    fn day_3() {
        let input = fs::read_to_string("../data/data/3.txt").unwrap();
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
        let input = aoc5::get_input();
        assert_eq!(0, aoc5::part_1(input).unwrap());
    }
}
