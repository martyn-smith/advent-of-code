// mod intcode;
// mod aoc1;
// mod aoc2;
// mod aoc3;
// mod aoc4;
// mod aoc5;
// mod aoc6;
// mod aoc2;
// mod aoc7;
mod aoc8;
// mod aoc9;
// mod aoc12;
// mod aoc13;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    let input = aoc8::get_input();
    aoc8::part_2(&input);
}
