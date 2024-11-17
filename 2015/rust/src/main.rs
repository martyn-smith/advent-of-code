#![allow(dead_code)]
mod aoc1;
mod aoc10;
mod aoc11;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc18;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc18::get_input();
    println!("{}", aoc18::part_2(&input));
}
