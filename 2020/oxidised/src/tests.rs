use super::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(aoc1::part_1(2020, &input), 224436);
    assert_eq!(aoc1::part_2(2020, &input), 303394260);
}
