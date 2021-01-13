mod intcode;
// mod aoc1;
// mod aoc2;
// mod aoc3;
// mod aoc4;
// mod aoc5;
//mod aoc6;
mod aoc7;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    //let opt = Opt::from_args();
    let input = aoc7::get_input();
    println!("{}", aoc7::part_1(&input));
}
