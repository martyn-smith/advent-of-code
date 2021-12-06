#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc6::get_input();
    println!("{}", aoc6::part_1(&input));
    println!("{}", aoc6::part_2(&input));
}
