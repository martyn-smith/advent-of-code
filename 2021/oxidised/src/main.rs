mod aoc1;

#[cfg(test)]
mod tests;

fn main() {
    let input = aoc1::get_input();
    println!("{}", aoc1::part_1(&input));
    println!("{}", aoc1::part_2(&input));
}
