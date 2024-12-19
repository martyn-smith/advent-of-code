use crate::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(2066446, aoc1::part_1(&input));
    assert_eq!(24931009, aoc1::part_2(&input));
}

#[test]
fn test_day_2() {
    let input = aoc2::get_input();
    assert_eq!(526, aoc2::part_1(&input));
    assert_eq!(566, aoc2::part_2(&input));
}

#[test]
fn test_day_4() {
    let input = aoc4::get_input();
    assert_eq!(2557, aoc4::part_1(&input));
    assert_eq!(1854, aoc4::part_2(&input));
}

#[test]
fn test_day_5() {
    let input = aoc5::get_input();
    assert_eq!(5991, aoc5::part_1(&input));
    assert_eq!(5479, aoc5::part_2(&input));
}

#[test]
fn test_day_6() {
    let input = aoc6::get_input();
    assert_eq!(5086, aoc6::part_1(&input));
    assert_eq!(1770, aoc6::part_2(&input));
}

#[test]
fn test_day_7() {
    let input = aoc7::get_input();
    assert_eq!(945512582195, aoc7::part_1(&input));
    assert_eq!(271691107779347, aoc7::part_2(&input));
}
