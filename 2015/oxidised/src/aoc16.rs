///
/// Advent of Code day 16: Good Sue Hunting
///
//use anyhow::Result;
use std::collections::HashMap;
//use std::fs;
//use regex::{Captures, Regex};

#[derive(Debug, Clone)]
pub struct Sue {
    id: usize,
    attrs: HashMap<String, usize>,
}

const ATTRS: [&str; 10] = [
    "children",
    "cats",
    "samoyeds",
    "pomeranians",
    "akitas",
    "vizslas",
    "goldfish",
    "trees",
    "cars",
    "perfumes",
];

const GT_ATTRS: [&str; 2] = ["cats", "trees"];

const LT_ATTRS: [&str; 2] = ["pomeranians", "goldfish"];

const REFSUE: &str = "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

impl Sue {
    fn new(description: &str) -> Self {
        let idx_srch = &description[4..description.find(':').unwrap()];
        let id = idx_srch.parse::<usize>().unwrap();
        let mut attrs = HashMap::new();
        for a in ATTRS {
            if let Some(idx) = description.find(a) {
                let start = description[idx..].find(':').unwrap() + idx + 2;
                let end = match description[idx..].find(',') {
                    Some(e) => e + idx,
                    None => description.len(),
                };
                let val = (description[start..end]).parse::<usize>().unwrap();
                attrs.insert(description[idx..start - 2].to_string(), val);
            }
        }
        Self {
            id,
            attrs
        }
    }

    fn hunt(&self, reference: &Sue) -> bool {
        for a in reference.attrs.keys() {
            if let Some(v) = self.attrs.get(a) {
                if v != reference.attrs.get(a).unwrap() {
                    return false;
                }
            }
        }
        true
    }

    fn range_hunt(&self, reference: &Sue) -> bool {
        for a in reference.attrs.keys() {
            if let Some(v) = self.attrs.get(a) {
                if a == "cats" || a == "trees" {
                    if v <= reference.attrs.get(a).unwrap() {
                        return false;
                    }
                } else if a == "pomeranians" || a == "goldfish" {
                    if v >= reference.attrs.get(a).unwrap() {
                        return false;
                    }
                } else if v != reference.attrs.get(a).unwrap() {
                    return false;
                }
            }
        }
        true
    }
}

pub fn get_input() -> Vec<Sue> {
    include_str!("../../data/16.txt")
        .lines()
        .map(Sue::new)
        .collect()
}

pub fn part_1(input: &[Sue]) -> usize {
    let refs = Sue::new(REFSUE);
    input
        .iter()
        .filter(|sue| sue.hunt(&refs))
        .map(|sue| sue.id)
        .next()
        .unwrap()
}

pub fn part_2(input: &[Sue]) -> usize {
    let refs = Sue::new(REFSUE);
    input
        .iter()
        .filter(|sue| sue.range_hunt(&refs))
        .map(|sue| sue.id)
        .next()
        .unwrap()
}
