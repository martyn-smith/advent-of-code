mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "3")]
    day: usize,
}

fn main() {
    let opt = Opt::from_args();
}
