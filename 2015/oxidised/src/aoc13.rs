use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

pub type JoyMap = HashMap<(String, String), isize>;

fn get_distance(seating: Vec<&String>, d: &JoyMap) -> isize {
    seating.windows(3)
                .map(|w| {
                          *d.get(&(w[1].to_string(), w[0].to_string())).unwrap()
                        + *d.get(&(w[1].to_string(), w[2].to_string())).unwrap()
                })
                .sum::<isize>()
    + d.get(&(seating[0].to_string(), seating[seating.len()-1].to_string())).unwrap()
    + d.get(&(seating[0].to_string(), seating[1].to_string())).unwrap()
    + d.get(&(seating[seating.len()-1].to_string(), seating[0].to_string())).unwrap()
    + d.get(&(seating[seating.len()-1].to_string(), seating[seating.len()-2].to_string())).unwrap()
}

pub fn get_input() -> (HashSet<String>, JoyMap) {
    /*
     * Bob would gain 83 happiness units by sitting next to Alice.
     */
    let mut psrns = HashSet::new();
    let mut joys = HashMap::new();
    let srch = Regex::new(r"(\w+)\s\w+\s(gain|lose)\s(\d+)(?:\s\w+){6}\s(\w+)\.").unwrap();
    for l in include_str!("../../data/13.txt").lines() {
        let s = srch.captures(l).unwrap();
        let p = s.get(1).unwrap().as_str().to_string();
        let gain = s.get(2).unwrap().as_str();
        let mut qty = s.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let nghbr = s.get(4).unwrap().as_str().to_string();
        if gain == "lose" {
            qty *= -1;
        }
        psrns.insert(p.clone());
        joys.insert((p, nghbr), qty);
    }
    (psrns, joys)
}

pub fn part_1(input: &(HashSet<String>, JoyMap)) -> isize {
    let (prsns, joys) = (&input.0, &input.1);
    prsns.iter().permutations(prsns.len())
            .map(|p| get_distance(p, &joys))
            .max()
            .unwrap()
}

pub fn part_2(input: &(HashSet<String>, JoyMap)) -> isize {
    let (mut prsns, mut joys) = (input.0.clone(), input.1.clone());
    let m = "me".to_string();
    for p in prsns.iter() {
        joys.insert((m.clone(), p.clone()), 0);
        joys.insert((p.clone(), m.clone()), 0);
    }
    prsns.insert(m);
    prsns.iter().permutations(prsns.len())
            .map(|p| get_distance(p, &joys))
            .max()
            .unwrap()
}
