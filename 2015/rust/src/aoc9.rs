use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub type DistanceMap = HashMap<(String, String), usize>;

fn get_distance(itinerary: Vec<&String>, d: &DistanceMap) -> usize {
    itinerary
        .windows(2)
        .map(|w| {
            if let Some(x) = d.get(&(w[0].to_string(), w[1].to_string())) {
                x
            } else if let Some(x) = d.get(&(w[1].to_string(), w[0].to_string())) {
                x
            } else {
                panic!("{:?} doesn't have a match in {:?}", w, d);
            }
        })
        .sum()
}

pub fn get_input() -> (HashSet<String>, DistanceMap) {
    //r"(\w+) to (\w+) = (\d+)
    let mut towns = HashSet::new();
    let mut distances = HashMap::new();
    let srch = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    for l in include_str!("../../data/9.txt").lines() {
        let s = srch.captures(l).unwrap();
        let start_town = s.get(1).unwrap().as_str().to_string();
        let dest_town = s.get(2).unwrap().as_str().to_string();
        let distance = s.get(3).unwrap().as_str().parse::<usize>().unwrap();
        towns.insert(start_town.clone());
        //apparently advance_by() is still in nightly, so...
        towns.insert(dest_town.clone());
        distances.insert((start_town, dest_town), distance);
    }
    (towns, distances)
}

pub fn part_1(input: &(HashSet<String>, DistanceMap)) -> usize {
    let (towns, distances) = (&input.0, &input.1);
    towns
        .iter()
        .permutations(towns.len())
        .map(|p| get_distance(p, distances))
        .min()
        .unwrap()
}

pub fn part_2(input: &(HashSet<String>, DistanceMap)) -> usize {
    let (towns, distances) = (&input.0, &input.1);
    towns
        .iter()
        .permutations(towns.len())
        .map(|p| get_distance(p, distances))
        .max()
        .unwrap()
}
