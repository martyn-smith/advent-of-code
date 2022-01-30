#![allow(dead_code)]
mod intcode;
mod aoc2;
mod aoc5;

#[cfg(test)]
mod tests;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    let input = aoc5::get_input();
    println!("{}", aoc5::part_1(&input));
    println!("{}", aoc5::part_2(&input));
}
