use super::*;

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
    assert_eq!(7912, aoc2::part_2(&input).unwrap());
}

#[test]
fn test_day_3() {
    let input = aoc3::get_input();
    assert_eq!(399, aoc3::part_1(&input));
    assert_eq!(15678, aoc3::part_2(&input));
}

#[test]
fn test_day_4() {
    let input = aoc4::get_input();
    assert_eq!(925, aoc4::part_1(&input));
    assert_eq!(607, aoc4::part_2(&input));
}

#[test]
fn test_day_5() {
    let input = aoc5::get_input();
    assert_eq!(13294380, aoc5::part_1(&input).unwrap());
    assert_eq!(11460760, aoc5::part_2(&input).unwrap());
}

#[test]
fn test_day_6() {
    let input = aoc6::get_input();
    assert_eq!(249308, aoc6::part_1(&input));
}

#[test]
fn test_day_7() {
    let input = aoc7::get_input();
    assert_eq!(22012, aoc7::part_1(&input));
}
#[test]
fn test_day_8() {
    let input = aoc8::get_input();
    assert_eq!(1905, aoc8::part_1(&input));
}
