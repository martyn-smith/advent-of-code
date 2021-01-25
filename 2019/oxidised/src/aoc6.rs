use std::collections::HashMap;
use std::fs;

fn get_orbits(orbits: &HashMap<String, Vec<String>>, planet: &str) -> usize {
    let planets = orbits.get(planet);
    match planets {
        Some(_) => planets
            .unwrap()
            .iter()
            .map(|p| get_orbits(orbits, p) + 1)
            .sum(),
        None => 0,
    }
}

pub fn get_input() -> HashMap<String, Vec<String>> {
    let input = fs::read_to_string("../data/6.txt").unwrap();
    let mut orbits = HashMap::<String, Vec<String>>::new();

    for l in input.lines() {
        let o: Vec<String> = l.split(')').map(|s| s.to_string()).collect();
        if let Some(s) = orbits.get_mut(&o[0]) {
            s.push(o[1].clone());
        } else {
            orbits.insert(o[0].clone(), vec![o[1].clone()]);
        }
    }
    orbits
}

pub fn part_1(orbits: &HashMap<String, Vec<String>>) -> usize {
    orbits.keys().map(|p| get_orbits(orbits, &p)).sum()
}

pub fn part_2(orbits: &HashMap<String, Vec<String>>) -> usize {
    let mut src = vec!["YOU".to_string()];
    let mut dst = vec!["SAN".to_string()];
    loop {
        let l = dst.last().unwrap().clone();
        if l == "COM" {
            break;
        }
        for (k, v) in orbits.iter() {
            if v.contains(&l) {
                dst.push(k.clone());
            }
        }
    }
    loop {
        let l1 = src.last().unwrap().clone();
        if l1 == "COM" {
            panic!("failed to find match");
        }
        for (i, d) in dst.iter().enumerate() {
            if d == &l1 {
                return (i - 1) + (src.len() - 2);
            }
        }
        for (k, v) in orbits.iter() {
            if v.contains(&l1) {
                src.push(k.clone());
            }
        }
    }
}
