use std::collections::HashSet;

fn get_substituted(subs: &[(String, String)], old: &str) -> HashSet<String> {
    let mut new = HashSet::<String>::new();
    for i in 0..(old.chars().count() - 1) {
        for (k, v) in subs {
            if *k == old[i..i + 1] {
                let mutant = format!("{}{}{}", &old[..i], v, &old[i + 1..]);
                new.insert(mutant);
            }
            if *k == old[i..=i + 1] {
                let mutant = format!("{}{}{}", &old[..i], v, &old[i + 2..]);
                new.insert(mutant);
            }
        }
    }
    new
}

pub fn get_input() -> (Vec<(String, String)>, String) {
    let mut input = include_str!("../../data/19.txt").split("\n\n");
    let subs = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut s = l.split(" => ");
            (s.next().unwrap().to_string(), s.next().unwrap().to_string())
        })
        .collect::<Vec<(String, String)>>();
    let initial = input.next().unwrap().to_string();
    (subs, initial)
}

pub fn part_1(input: &(Vec<(String, String)>, String)) -> usize {
    let new = get_substituted(&input.0, &input.1);
    new.len()
}
