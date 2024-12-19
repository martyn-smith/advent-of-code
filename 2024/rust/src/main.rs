#![allow(dead_code)]

#[cfg(test)]
mod tests;

mod aoc1;
mod aoc2;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc11;

fn main() {
    let input = aoc5::get_input();
    println!("{}", aoc5::part_1(&input));
    println!("{}", aoc5::part_2(&input));
}
