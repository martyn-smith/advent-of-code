#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc7::get_input();
    println!("{}", aoc7::part_1(&input));
    println!("{}", aoc7::part_2(&input));
}
