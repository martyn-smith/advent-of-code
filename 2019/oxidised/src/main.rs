#[allow(dead_code)]
mod intcode;
// mod aoc1;
// mod aoc2;
// mod aoc3;
// mod aoc4;
// mod aoc5;
// mod aoc6;
// mod aoc2;
// mod aoc7;
// mod aoc8;
// mod aoc9;
// mod aoc10;
mod aoc11;
// mod aoc12;
// mod aoc13;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    //1489 < x < 6015
    let input = aoc11::get_input();
    println!("{}", aoc11::part_1(&input));
    aoc11::part_2(&input);
}
