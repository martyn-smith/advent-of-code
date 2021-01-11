mod aoc1;
mod aoc2;
// mod aoc3;
mod aoc4;
mod aoc5;

#[cfg(test)]
mod tests {
    use super::aoc1;
    use super::aoc2;
    // use super::aoc3;
    use super::aoc4;
    use super::aoc5;

    #[test]
    fn test_day_1() {
        let input = aoc1::get_input();
        assert_eq!(3454942, aoc1::part_1(&input));   
        assert_eq!(5179544, aoc1::part_2(&input));
    }

    #[test]
    fn test_day_2() {
        let input = aoc2::get_input();
        assert_eq!(3409710, aoc2::part_1(&input).unwrap());
        assert_eq!(7912, aoc2::part_2(&input, 19690720).unwrap());
    }

//     #[test]
//     fn test_day_3() {
//         // let input = fs::read_to_string("../data/data/3.txt").unwrap();
//         // let entries = aoc3::parse_line(input.lines().nth(0).unwrap().to_string());
//         // println!("{:?}", entries);
//         // assert!(false);
//     }

    #[test]
    fn test_day_4() {
        let input = aoc4::get_input();
        assert_eq!(925, aoc4::part_1(input));
    }

    #[test]
    fn test_day_5() {
        let input = aoc5::get_input();
        assert_eq!(13294380, aoc5::part_1(input).unwrap());
    }
}
