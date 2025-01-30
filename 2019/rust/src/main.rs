#![allow(dead_code)]
mod aoc1;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc17;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod intcode;

#[cfg(test)]
mod tests;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    let input = aoc14::get_input();
    println!("{}", aoc14::part_1(&input));
    println!("{}", aoc14::part_2(&input));
}
