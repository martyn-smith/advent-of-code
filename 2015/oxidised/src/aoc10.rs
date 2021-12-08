///
/// Advent of Code day 10: speak-and-say
///

fn play_game(input: &str, reps: usize) -> usize {
    let mut prev = input.to_string();
    let mut next = String::new();
    let mut cursor ;
    let mut counter ;
    for _ in 0..reps {
        next.clear();
        cursor = None;
        counter = 0;
        for c in prev.chars() {
            if let Some(d) = cursor {
                if c == d {
                    counter += 1;
                } else {
                    // println!("end of run: {}{}", counter, d);
                    next += &format!("{}{}", counter, d)[..];
                    cursor = Some(c);
                    counter = 1;
                }
            } else {
                cursor = Some(c);
                counter = 1;
            }
        }
        next += &format!("{}{}", counter, cursor.unwrap())[..];
        prev = next.clone();
    }
    next.len()
}

pub fn get_input() -> String {
    "1321131112".to_string()
}

pub fn part_1(input: &str) -> usize {
    play_game(input, 40)
}

pub fn part_2(input: &str) -> usize {
    play_game(input, 50)
}
