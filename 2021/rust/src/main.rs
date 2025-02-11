#![allow(dead_code)]
mod alu;
mod aoc1;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
//mod aoc15;
mod aoc16;
mod aoc21;
mod aoc22;
mod aoc24;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc16::get_input();
    println!("{}", aoc16::part_1(&input));
    println!("{}", aoc16::part_2(&input));
}
