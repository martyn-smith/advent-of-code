#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc4::get_input();
    println!("{}", aoc4::part_1(&input));
    println!("{}", aoc4::part_2(&input));
}
