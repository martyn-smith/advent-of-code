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

// ISSUE: this is rather slow.
//#[test]
//fn test_day_4() {
//    let input = aoc4::get_input();
//    assert_eq!(346386, aoc4::part_1(&input));
//    assert_eq!(9958218, aoc4::part_2(&input));
//}

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
fn test_day_8() {
    let input = aoc8::get_input();
    assert_eq!(1342, aoc8::part_1(&input));
    assert_eq!(2074, aoc8::part_2(&input));
}

#[test]
fn test_day_9() {
    let input = aoc9::get_input();
    assert_eq!(251, aoc9::part_1(&input));
    assert_eq!(898, aoc9::part_2(&input));
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

#[test]
fn test_day_13() {
    let input = aoc13::get_input();
    assert_eq!(733, aoc13::part_1(&input));
    assert_eq!(725, aoc13::part_2(&input));
}

#[test]
fn test_day_14() {
    let input = aoc14::get_input();
    assert_eq!(2640, aoc14::part_1(&input));
    assert_eq!(1102, aoc14::part_2(&input));
}

#[test]
fn test_day_15() {
    let input = aoc15::get_input();
    assert_eq!(18965440, aoc15::part_1(&input));
    assert_eq!(15862900, aoc15::part_2(&input));
}

#[test]
fn test_day_16() {
    let input = aoc16::get_input();
    assert_eq!(103, aoc16::part_1(&input));
    assert_eq!(405, aoc16::part_2(&input));
}

#[test]
fn test_day_18() {
    let input = aoc18::get_input();
    assert_eq!(768, aoc18::part_1(&input));
    assert_eq!(781, aoc18::part_2(&input));
}

#[test]
fn test_day_19() {
    let input = aoc19::get_input();
    assert_eq!(576, aoc19::part_1(&input));
}

#[test]
fn test_day_21() {
    let input = aoc21::get_input();
    assert_eq!(111, aoc21::part_1(&input));
    assert_eq!(188, aoc21::part_2(&input));
}

#[test]
fn test_day_23() {
    let input = aoc23::get_input();
    assert_eq!(184, aoc23::part_1(&input));
    assert_eq!(231, aoc23::part_2(&input));
}

#[test]
fn test_day_24() {
    let input = aoc24::get_input();
    assert_eq!(10723906903, aoc24::part_1(&input));
    assert_eq!(74850409, aoc24::part_2(&input));
}
