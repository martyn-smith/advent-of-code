// mod aoc1;
// mod aoc2;
// mod aoc3;
// mod aoc4;
// mod aoc5;
mod aoc6;

// use structopt::StructOpt;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short, long, default_value = "6")]
//     day: usize,
// }

fn main() {
    //let opt = Opt::from_args();
    let input = aoc6::get_input();
    println!("{}", aoc6::part_1(input));
}
