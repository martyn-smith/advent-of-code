#[cfg(test)]
mod tests;

mod aoc1;
mod aoc2;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;

fn main() {
    let input = aoc6::get_input();
    println!("{}", aoc6::part_1(&input));
}
