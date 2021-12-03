mod aoc1;
mod aoc2;
mod aoc3;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc3::get_input();
    println!("{}", aoc3::part_1(&input));
    println!("{}", aoc3::part_2(&input));
}
