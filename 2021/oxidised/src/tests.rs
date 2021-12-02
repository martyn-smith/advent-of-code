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
