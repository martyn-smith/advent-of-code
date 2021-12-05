#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc5::get_input();
    println!("{}", aoc5::part_1(&input));
    println!("{}", aoc5::part_2(&input));
}
