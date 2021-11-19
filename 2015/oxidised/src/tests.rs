use super::*;

#[test]
fn test_day_1() {
    let input = aoc1::get_input();
    assert_eq!(aoc1::part_1(&input), 280);
    assert_eq!(aoc1::part_2(&input), 1797);
}

#[test]
fn test_day_2() {
    let input = aoc2::get_input();
    assert_eq!(aoc2::part_1(&input), 1598415);
    assert_eq!(aoc2::part_2(&input), 3812909);
}

#[test]
fn test_day_3() {
    let input = aoc3::get_input();
    assert_eq!(aoc3::part_1(&input), 2572);
    assert_eq!(aoc3::part_2(&input), 2631);
}
