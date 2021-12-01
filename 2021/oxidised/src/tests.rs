use super::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(1301, aoc1::part_1(&input));
    assert_eq!(1346, aoc1::part_2(&input));
}
