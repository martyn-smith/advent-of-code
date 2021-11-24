use super::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(280, aoc1::part_1(&input));
    assert_eq!(1797, aoc1::part_2(&input));
}

#[test]
fn test_day_2() {
    let input = aoc2::get_input();
    assert_eq!(1598415, aoc2::part_1(&input));
    assert_eq!(3812909, aoc2::part_2(&input));
}

#[test]
fn test_day_3() {
    let input = aoc3::get_input();
    assert_eq!(2572, aoc3::part_1(&input));
    assert_eq!(2631, aoc3::part_2(&input));
}
/*
#[test]
fn test_day_4() {
    let input = aoc4::get_input();
    assert_eq!(346386, aoc4::part_1(&input));
    assert_eq!(9958218, aoc4::part_2(&input));
}
*/
#[test]
fn test_day_6() {
    let input = aoc6::get_input();
    assert_eq!(569999, aoc6::part_1(&input));
    assert_eq!(17836115, aoc6::part_2(&input));
}

#[test]
fn test_day_7() {
    let input = aoc7::get_input();
    assert_eq!(3176, aoc7::part_1(&input));
    assert_eq!(14710, aoc7::part_2(&input));
}

#[test]
fn test_day_10() {
    let input = aoc10::get_input();
    assert_eq!(492982, aoc10::part_1(&input));
    assert_eq!(6989950, aoc10::part_2(&input));
}

#[test]
fn test_day_11() {
    let input = aoc11::get_input();
    assert_eq!("cqjxxyzz", &aoc11::part_1(&input));
    assert_eq!("cqkaabcc", &aoc11::part_2(&input));
}


