use super::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(1301, aoc1::part_1(&input));
    assert_eq!(1346, aoc1::part_2(&input));
}

#[test]
fn test_day_2() {
    let input = aoc2::get_input();
    assert_eq!(1561344, aoc2::part_1(&input));
    assert_eq!(1848454425, aoc2::part_2(&input));
}

#[test]
fn test_day_3() {
    let input = aoc3::get_input();
    assert_eq!(3277364, aoc3::part_1(&input));
    assert_eq!(5736383, aoc3::part_2(&input));
}

#[test]
fn test_day_4() {
    let input = aoc4::get_input();
    assert_eq!(38913, aoc4::part_1(&input));
    assert_eq!(16836, aoc4::part_2(&input));
}

#[test]
fn test_day_5() {
    let input = aoc5::get_input();
    assert_eq!(8111, aoc5::part_1(&input));
    assert_eq!(22088, aoc5::part_2(&input));
}

#[test]
fn test_day_6() {
    let input = aoc6::get_input();
    assert_eq!(394994, aoc6::part_1(&input));
    assert_eq!(1765974267455, aoc6::part_2(&input));
}

#[test]
fn test_day_7() {
    let input = aoc7::get_input();
    assert_eq!(357353, aoc7::part_1(&input));
    assert_eq!(104822130, aoc7::part_2(&input));
}

#[test]
fn test_day_8() {
    let input = aoc8::get_input();
    assert_eq!(525, aoc8::part_1(&input));
    assert_eq!(1083859, aoc8::part_2(&input));
}

#[test]
fn test_day_9() {
    let input = aoc9::get_input();
    assert_eq!(458, aoc9::part_1(&input));
    assert_eq!(1391940, aoc9::part_2(&input));
}

#[test]
fn test_day_10() {
    let input = aoc10::get_input();
    assert_eq!(367059, aoc10::part_1(&input));
    assert_eq!(1952146692, aoc10::part_2(&input));
}

#[test]
fn test_day_11() {
    let input = aoc11::get_input();
    assert_eq!(1665, aoc11::part_1(&input));
    assert_eq!(235, aoc11::part_2(&input));
}

#[test]
fn test_day_12() {
    let input = aoc12::get_input();
    assert_eq!(5457, aoc12::part_1(&input));
    assert_eq!(128506, aoc12::part_2(&input));
}

#[test]
fn test_day_13() {
    let input = aoc13::get_input();
    assert_eq!(666, aoc13::part_1(&input));
}

#[test]
fn test_day_14() {
    let input = aoc14::get_input();
    assert_eq!(4244, aoc14::part_1(&input));
    assert_eq!(4807056953866, aoc14::part_2(&input));
}

#[test]
fn test_day_21() {
    let input = aoc21::get_input();
    assert_eq!(920079, aoc21::part_1(&input));
}
