///
/// Advent of Code day 12: Spelunking
///


use std::collections::{HashMap, HashSet};

fn is_small(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase())
}

fn find_paths(current: &str, list: &HashMap<String, Vec<String>>, mut visited: HashSet<String>) -> Option<usize> {
    if current == "end" {
        Some(1)
    } else if is_small(current) && visited.contains(current) {
        None
    } else {
        visited.insert(current.to_string());
        let caves = list.get(current).unwrap();
        Some(caves.iter()
            .filter_map(|c| find_paths(c, list, visited.clone()))
            .sum())
    }
}

fn find_paths_2(current: &str, list: &HashMap<String, Vec<String>>, mut visited: HashSet<String>, used_lifeline: bool) -> Option<usize> {
    if current == "end" {
        Some(1)
    } else if current == "start" && visited.contains(current) {
        None
    } else if is_small(current) && visited.contains(current) && used_lifeline {
        None
    } else {
        let used_lifeline = used_lifeline || (is_small(current) && visited.contains(current));
        visited.insert(current.to_string());
        let caves = list.get(current).unwrap();
        Some(caves.iter()
            .filter_map(|c| find_paths_2(c, list, visited.clone(), used_lifeline))
            .sum())
    }
}

pub fn get_input() -> HashMap<String, Vec<String>> {
    let mut input : HashMap<String, Vec<String>> = HashMap::new();
    for l in include_str!("../../data/12.txt").lines() {
        let mut s = l.split('-');
        let a = s.next().unwrap().to_string();
        let b = s.next().unwrap().to_string();
        if let Some(p) = input.get_mut(&a) {
            p.push(b.clone());
        } else {
            input.insert(a.clone(), vec![b.clone()]);
        }
        if let Some(p) = input.get_mut(&b) {
                    p.push(a.clone());
                } else {
                    input.insert(b.clone(), vec![a.clone()]);
                }
            }
    input
}

pub fn part_1(input: &HashMap<String, Vec<String>>) -> usize {
    let visited = HashSet::new();
    find_paths("start", input, visited).unwrap()
}

pub fn part_2(input: &HashMap<String, Vec<String>>) -> usize {
    let visited = HashSet::new();
    find_paths_2("start", input, visited, false).unwrap()
}
