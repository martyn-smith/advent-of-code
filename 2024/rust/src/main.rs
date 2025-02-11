#![allow(dead_code)]

#[cfg(test)]
mod tests;

mod aoc1;
mod aoc11;
mod aoc2;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;

fn main() {
    let input = aoc8::get_input();
    println!("{}", aoc8::part_1(&input));
    println!("{}", aoc8::part_2(&input));
}
