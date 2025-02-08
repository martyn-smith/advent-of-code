use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::{thread::sleep, time::Duration};

const OPEN: &str = "Rn";
const CLOSE: &str = "Ar";

fn get_substituted(rules: &[[String; 2]], old: &str) -> HashSet<String> {
    let mut new = HashSet::<String>::new();
    for i in 0..(old.chars().count() - 1) {
        for [k, v] in rules {
            if k == &old[i..i + 1] {
                let mutant = format!("{}{}{}", &old[..i], v, &old[i + 1..]);
                new.insert(mutant);
            } else if k == &old[i..=i + 1] {
                let mutant = format!("{}{}{}", &old[..i], v, &old[i + 2..]);
                new.insert(mutant);
            }
        }
    }
    new
}

/*
 * Greedy reduction gets you stuck on a Ca(F) sequence,
 * adding a manual expansion step leads to an answer of 209, 2 too high.
 * Not really sure if there's a deterministic way around that without
 * hardcoding in that knowledge? Or we could use a GA,
 * but that's not exactly performant.
 */
fn do_substitutions(rules: &[[String; 2]], old: &str) -> Result<usize> {
    let mut rules = rules.to_owned();
    let mut tgt = old.to_string();
    let mut mutations = 0;
    rules.sort_by_key(|r| r[1].len());

    while tgt != "e" {
        let mut rnd_mutations = 0;
        for [k, v] in rules.iter().rev() {
            rnd_mutations = tgt.matches(v).count();
            mutations += rnd_mutations;
            tgt = tgt.replace(v, k);
            //sleep(Duration::from_millis(100));
        }
        println!("===\n{}\n{} {}\n===", &tgt, mutations, rnd_mutations);

        //sleep(Duration::from_millis(100));
    }
    Ok(mutations)
}

pub fn get_input() -> (Vec<[String; 2]>, String) {
    let mut input = include_str!("../../data/19.txt").split("\n\n");
    let subs = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut s = l.split(" => ");
            [
                s.next().unwrap().to_string(),
                s.next().unwrap().replace("Rn", "(").replace("Ar", ")"),
            ]
        })
        .collect::<Vec<_>>();
    let initial = input.next().unwrap().replace("Rn", "(").replace("Ar", ")");
    (subs, initial)
}

pub fn part_1(input: &(Vec<[String; 2]>, String)) -> usize {
    let new = get_substituted(&input.0, &input.1);
    new.len()
}

pub fn part_2(input: &(Vec<[String; 2]>, String)) -> usize {
    do_substitutions(&input.0, &input.1).unwrap()
}
