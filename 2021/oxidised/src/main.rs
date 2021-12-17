#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc14::get_input();
    println!("{}", aoc14::part_1(&input));
    println!("{}", aoc14::part_2(&input));
}
